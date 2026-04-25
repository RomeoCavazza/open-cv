#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

import json
import shutil

# --- CONFIGURATION ---
SPECIAL_WORDS = {
    "api": "API", "ci/cd": "CI/CD", "data": "Data", "devops": "DevOps",
    "finops": "FinOps", "full": "Full", "hpc": "HPC", "ia": "IA",
    "iot": "IoT", "llm": "LLM", "ml": "ML", "rag": "RAG",
    "sirh": "SIRH", "stack": "Stack",
}

# --- LOGIQUE DE PRETTIFY ---

def prettify_name(value: str) -> str:
    value = value.strip()
    if not value: return value
    tokens = value.split()
    letters = [char for char in value if char.isalpha()]
    is_all_caps = bool(letters) and all(char.isupper() for char in letters)
    if is_all_caps and len(tokens) >= 2:
        head = [token.capitalize() for token in tokens[:-1]]
        return " ".join([*head, tokens[-1].upper()])
    return value

def prettify_title(value: str) -> str:
    value = value.strip()
    if not value: return value
    mapped: list[str] = []
    normalized = value.replace(" - ", " — ")
    for part in re.split(r"(\s+|/)", normalized.lower()):
        if not part or part.isspace() or part == "/":
            mapped.append(part)
            continue
        mapped.append(SPECIAL_WORDS.get(part, part.capitalize()))
    return "".join(mapped)

def order_contacts(lines: list[str]) -> list[str]:
    def rank(value: str) -> tuple[int, str]:
        lowered = value.lower()
        if "github.com" in lowered: return (5, lowered)
        if lowered.startswith("in/") or "linkedin" in lowered: return (4, lowered)
        if "www." in lowered or lowered.startswith("http"): return (3, lowered)
        if "@" in lowered: return (2, lowered)
        if re.match(r"^\+?[\d.\s()-]+$", value): return (1, lowered)
        return (0, lowered)
    clean_lines = [l.strip() for l in lines if l.strip()]
    return sorted(clean_lines, key=rank)

def clean_wrapped_text(lines: list[str]) -> str:
    text = " ".join([l.strip() for l in lines if l.strip()])
    text = re.sub(r"\s+,", ",", text)
    text = re.sub(r"\s+([.;:])", r"\1", text)
    return re.sub(r"\s{2,}", " ", text).strip()

# --- PARSING ---

def parse_pdf(path: Path) -> dict:
    try:
        result = subprocess.run(["pdftotext", "-raw", "-nopgbrk", str(path), "-"],
                                 check=True, capture_output=True, text=True)
        lines = result.stdout.splitlines()
        return {"name": path.stem, "sections": [("IMPORT", lines)]}
    except Exception as e:
        print(f"Erreur PDF {path}: {e}")
        return {}

def parse_template_md(text: str) -> dict:
    parts = re.split(r"\n---\n", text)
    data = {"header": [], "sections": []}
    data["header"] = parts[0].splitlines()
    for part in parts[1:]:
        lines = part.splitlines()
        if not lines: continue
        title_idx = next((i for i, l in enumerate(lines) if l.strip()), None)
        if title_idx is None: continue
        section_title = lines[title_idx].strip().upper()
        section_body = lines[title_idx+1:]
        data["sections"].append((section_title, section_body))
    return data

# --- RENDERING ---

def render_markdown_cv(data: dict) -> str:
    if not data: return ""
    res = []
    header_lines = [l.strip() for l in data.get("header", []) if l.strip()]
    if header_lines:
        res.append(prettify_name(header_lines[0]))
        if len(header_lines) > 1:
            res.append(prettify_title(header_lines[1]))
        res.append("")
        for l in header_lines[2:]:
            res.append(l)
    for title, body in data.get("sections", []):
        res.append("\n---\n")
        res.append(title)
        res.append("")
        if "CONTACT" in title:
            res.extend(order_contacts(body))
        else:
            joined_body = "\n".join(body).strip("\n")
            res.append(joined_body)
    return "\n".join(res).rstrip() + "\n"

# --- COMMANDS ---

def command_init(job_id: str, force: bool = False):
    root = Path(__file__).parent.parent.parent
    data_dir = root / "data"
    instances_dir = data_dir / "instances" / job_id
    templates_dir = data_dir / "templates"
    
    # Path for offer
    offer_path = data_dir / "offres" / "raw" / f"{job_id}.md"
    
    # 1. Existence check for source offer
    source_uri = None
    if offer_path.exists():
        source_uri = str(offer_path.relative_to(root))
    else:
        print(f"⚠️  Warning: Source offer not found: {offer_path}")
        
    # 2. Existence check for instance folder (Safeguard)
    if instances_dir.exists() and not force:
        print(f"❌ Error: Instance '{job_id}' already exists. Use --force to overwrite.")
        sys.exit(1)
        
    if instances_dir.exists() and force:
        print(f"🔄 Overwriting existing instance '{job_id}'...")
        shutil.rmtree(instances_dir)
        
    instances_dir.mkdir(parents=True)
    
    # 3. Copy defaults
    shutil.copy(templates_dir / "resume.json", instances_dir / "resume.json")
    shutil.copy(templates_dir / "cover-letter.json", instances_dir / "cover-letter.json")
    
    # 4. Generate meta.json (Lean format)
    meta = {
        "job_id": job_id,
        "source_offer": source_uri,
        "status": "draft",
        "version": "v1"
    }
    (instances_dir / "meta.json").write_text(json.dumps(meta, indent=2, ensure_ascii=False))
    print(f"✅ Instance '{job_id}' initialized.")

def command_render(job_id: str):
    root = Path(__file__).parent.parent.parent
    instance_dir = root / "data" / "instances" / job_id
    web_dir = root / "engines" / "web"
    
    if not instance_dir.exists():
        print(f"Error: Instance '{job_id}' not found. Run 'init' first.")
        sys.exit(1)
        
    # Preview bridge: copy to web folders
    shutil.copy(instance_dir / "resume.json", web_dir / "resume" / "data.json")
    shutil.copy(instance_dir / "cover-letter.json", web_dir / "cover-letter" / "data.json")
    print(f"🚀 Preview activated for '{job_id}'.")

def command_clean(input_path: Path):
    files = [input_path] if input_path.is_file() else list(input_path.rglob("*.md"))
    for f in files:
        if f.suffix.lower() == ".md":
            print(f"Standardisation: {f.name}")
            data = parse_template_md(f.read_text(encoding="utf-8"))
            f.write_text(render_markdown_cv(data), encoding="utf-8")

def command_init_all(force: bool = False):
    root = Path(__file__).parent.parent.parent
    liste_path = root / "data" / "offres" / "liste.json"
    
    if not liste_path.exists():
        print(f"Error: {liste_path} not found.")
        return

    with open(liste_path, 'r') as f:
        data = json.load(f)
    
    entries = data.get("entries", [])
    print(f"📦 Found {len(entries)} entries. Starting batch initialization...")
    
    count = 0
    for entry in entries:
        job_id = entry.get("job_id")
        if job_id:
            # We call command_init but handle the exit(1) to not break the loop
            try:
                command_init(job_id, force=force)
                count += 1
            except SystemExit:
                # This happens if it exists and force is False
                continue
    
    print(f"\n✨ Batch complete. {count} instances are ready in data/instances/")

def main():
    parser = argparse.ArgumentParser(description="CV Tool: Instance Management & Formatting.")
    subparsers = parser.add_subparsers(dest="command", help="Commands")

    # init
    parser_init = subparsers.add_parser("init", help="Initialize a new job instance")
    parser_init.add_argument("job_id", help="The ID of the job")
    parser_init.add_argument("-f", "--force", action="store_true", help="Overwrite existing instance")

    # init-all
    parser_init_all = subparsers.add_parser("init-all", help="Initialize all instances from liste.json")
    parser_init_all.add_argument("-f", "--force", action="store_true", help="Overwrite existing instances")

    # render
    parser_render = subparsers.add_parser("render", help="Set an instance as the active preview")
    parser_render.add_argument("job_id", help="The ID of the job instance")

    # clean
    parser_clean = subparsers.add_parser("clean", help="Standardize Markdown CV files")
    parser_clean.add_argument("path", type=Path, help="File or directory to clean")

    args = parser.parse_args()

    if args.command == "init":
        command_init(args.job_id, args.force)
    elif args.command == "init-all":
        command_init_all(args.force)
    elif args.command == "render":
        command_render(args.job_id)
    elif args.command == "clean":
        command_clean(args.path)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
