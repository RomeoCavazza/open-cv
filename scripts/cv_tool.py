#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

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
    # On ne trie que si on a bien des lignes de contact
    clean_lines = [l.strip() for l in lines if l.strip()]
    return sorted(clean_lines, key=rank)

def clean_wrapped_text(lines: list[str]) -> str:
    # On ne fusionne que si c'est vraiment un paragraphe (résumé)
    text = " ".join([l.strip() for l in lines if l.strip()])
    text = re.sub(r"\s+,", ",", text)
    text = re.sub(r"\s+([.;:])", r"\1", text)
    return re.sub(r"\s{2,}", " ", text).strip()

# --- PARSING ---

def parse_pdf(path: Path) -> dict:
    # Utilise pdftotext pour une extraction brute
    try:
        result = subprocess.run(["pdftotext", "-raw", "-nopgbrk", str(path), "-"],
                                 check=True, capture_output=True, text=True)
        lines = result.stdout.splitlines()
        # On délègue au parser de lignes (logique simplifiée)
        return {"name": path.stem, "sections": [("IMPORT", lines)]}
    except Exception as e:
        print(f"Erreur PDF {path}: {e}")
        return {}

def parse_template_md(text: str) -> dict:
    # Découpage par les séparateurs de sections "---"
    # On utilise un split qui garde les lignes vides à l'intérieur des parties
    parts = re.split(r"\n---\n", text)
    
    data = {"header": [], "sections": []}
    
    # Header (Nom, Titre, Résumé, Metadata)
    data["header"] = parts[0].splitlines()
    
    # Sections (CONTACT, EXPERIENCES, etc.)
    for part in parts[1:]:
        lines = part.splitlines()
        if not lines: continue
        # On cherche la première ligne non vide pour le titre de section
        title_idx = next((i for i, l in enumerate(lines) if l.strip()), None)
        if title_idx is None: continue
        
        section_title = lines[title_idx].strip().upper()
        section_body = lines[title_idx+1:]
        data["sections"].append((section_title, section_body))
            
    return data

# --- RENDERING ---

def render_cv(data: dict) -> str:
    if not data: return ""
    
    res = []
    # Header
    header_lines = [l.strip() for l in data.get("header", []) if l.strip()]
    if header_lines:
        res.append(prettify_name(header_lines[0]))
        if len(header_lines) > 1:
            res.append(prettify_title(header_lines[1]))
        res.append("")
        # Reste du header (résumé / metadata)
        for l in header_lines[2:]:
            res.append(l)
    
    # Sections (Ordre original préservé)
    for title, body in data.get("sections", []):
        res.append("\n---\n")
        res.append(title)
        res.append("")
        
        if "CONTACT" in title:
            # Nettoyage et tri spécial pour les contacts
            res.extend(order_contacts(body))
        else:
            # On garde le corps tel quel (y compris les lignes vides !)
            # Mais on strip les lignes inutiles au début/fin du bloc
            joined_body = "\n".join(body).strip("\n")
            res.append(joined_body)
        
    return "\n".join(res).rstrip() + "\n"

def main():
    parser = argparse.ArgumentParser(description="CV Tool: Secure Import and Reformat.")
    parser.add_argument("input", type=Path)
    args = parser.parse_args()

    files = [args.input] if args.input.is_file() else list(args.input.rglob("*.md"))
    
    for f in files:
        if f.suffix.lower() == ".md":
            print(f"Standardisation (Safe Mode): {f.name}")
            data = parse_template_md(f.read_text(encoding="utf-8"))
            f.write_text(render_cv(data), encoding="utf-8")

if __name__ == "__main__":
    main()
