#!/usr/bin/env python3
from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys
import time
import unicodedata
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable
from urllib.parse import urlparse


def ensure_runtime() -> None:
    """Re-exec with a GCC runtime in LD_LIBRARY_PATH if curl_cffi needs libstdc++."""
    try:
        import curl_cffi  # noqa: F401
        import bs4  # noqa: F401
        import markdownify  # noqa: F401
        import yaml  # noqa: F401
    except ImportError as exc:
        if "libstdc++.so.6" not in str(exc) or os.environ.get("_SCRAPER_LD_FIXED") == "1":
            raise

        candidates = sorted(glob.glob("/nix/store/*gcc*-lib/lib"))
        for candidate in candidates:
            if Path(candidate, "libstdc++.so.6").exists():
                env = os.environ.copy()
                env["_SCRAPER_LD_FIXED"] = "1"
                current = env.get("LD_LIBRARY_PATH", "")
                env["LD_LIBRARY_PATH"] = f"{candidate}:{current}" if current else candidate
                os.execve(sys.executable, [sys.executable, *sys.argv], env)
        raise


ensure_runtime()

from bs4 import BeautifulSoup  # noqa: E402
from curl_cffi import requests as curl_requests  # noqa: E402
from markdownify import markdownify as html_to_markdown  # noqa: E402
import yaml  # noqa: E402


ENTRY_RE = re.compile(r"- \[(.*?)\]\((https?://[^)]+)\)")
DEFAULT_CONFIG_PATH = "/home/tco/Bureau/alternance/config/scrape-offres.yaml"
DEFAULT_LIST_PATH = "/home/tco/Bureau/alternance/offres/liste.md"
DEFAULT_OUTPUT_DIR = "/home/tco/Bureau/alternance/offres/offres"
DEFAULT_HTML_DIR = None
DEFAULT_REPORT_PATH = None
REQUEST_HEADERS = {
    "Accept-Language": "fr-FR,fr;q=0.95,en-US;q=0.85,en;q=0.8",
    "Cache-Control": "no-cache",
    "Pragma": "no-cache",
}
GENERIC_SELECTORS = [
    "main",
    "article",
    '[role="main"]',
    "#main-content",
    "#content",
    ".main-content",
    ".content",
    ".job-details",
    ".job-description",
    "body",
]
ALTERNATIVE_SOURCE_BY_URL = {
    "https://epitech.jobteaser.com/en/job-offers/cddf9910-31e8-411e-8cba-40a2c7281d08-societe-generale-developpeur-full-stack-agile": "https://www.welcometothejungle.com/fr/companies/societe-generale/jobs/developpeur-full-stack-agile_fontenay-sous-bois_SG_lJra2Wa",
}
DOMAIN_SELECTORS = {
    "epitech.jobteaser.com": [
        "main#job-ad-detail-content",
        "#pageMainContent",
        "main",
    ],
    "www.safran-group.com": [
        "div.details-offers-page",
        "#main-content",
        "main",
    ],
    "safran-group.com": [
        "div.details-offers-page",
        "#main-content",
        "main",
    ],
    "www.welcometothejungle.com": [
        '[data-testid="job-section-description"]',
        '[data-testid="job-metadata-block"]',
        "article",
        "body",
    ],
    "welcometothejungle.com": [
        '[data-testid="job-section-description"]',
        '[data-testid="job-metadata-block"]',
        "article",
        "body",
    ],
}
TAG_NAMES_TO_DROP = [
    "script",
    "style",
    "noscript",
    "template",
    "svg",
    "path",
    "meta",
    "link",
    "img",
]


@dataclass
class OfferEntry:
    list_title: str
    url: str
    slug: str
    category: str | None = None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Scrape toutes les offres depuis un Markdown ou un JSON piloté par YAML."
    )
    parser.add_argument(
        "--config",
        default=DEFAULT_CONFIG_PATH,
        help="Fichier YAML de configuration.",
    )
    parser.add_argument(
        "--list",
        default=None,
        help="Alias historique pour --input-path quand le format est markdown.",
    )
    parser.add_argument(
        "--input-path",
        default=None,
        help="Chemin du fichier d'entrée (Markdown ou JSON).",
    )
    parser.add_argument(
        "--input-format",
        choices=["markdown", "json"],
        default=None,
        help="Format du fichier d'entrée.",
    )
    parser.add_argument(
        "--output-dir",
        default=None,
        help="Dossier de sortie pour les fichiers .md.",
    )
    parser.add_argument(
        "--html-dir",
        default=None,
        help="Dossier de sortie pour les snapshots HTML. Omettre ou mettre null pour désactiver.",
    )
    parser.add_argument(
        "--report",
        default=None,
        help="Chemin du rapport JSON. Omettre ou mettre null pour désactiver.",
    )
    parser.add_argument(
        "--legacy-dir",
        default=None,
        help="Dossier des anciens exports .md servant de fallback.",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=None,
        help="Timeout HTTP par requête, en secondes.",
    )
    parser.add_argument(
        "--retries",
        type=int,
        default=None,
        help="Nombre maximum de tentatives par URL.",
    )
    parser.add_argument(
        "--overwrite",
        action="store_true",
        default=None,
        help="Réécrit les fichiers .md et .html déjà présents.",
    )
    return parser.parse_args()


def slugify(value: str) -> str:
    normalized = unicodedata.normalize("NFKD", value)
    ascii_only = normalized.encode("ascii", "ignore").decode("ascii").lower()
    ascii_only = ascii_only.replace("&", " and ")
    ascii_only = re.sub(r"[^a-z0-9]+", "_", ascii_only)
    return re.sub(r"_+", "_", ascii_only).strip("_") or "offre"


def build_entries(raw_entries: list[dict[str, Any]]) -> list[OfferEntry]:
    used_slugs: set[str] = set()
    entries: list[OfferEntry] = []
    for item in raw_entries:
        title = str(item["title"]).strip()
        url = str(item["url"]).strip()
        category = item.get("category")
        explicit_slug = item.get("slug")
        base_slug = slugify(str(explicit_slug) if explicit_slug else title)
        slug = base_slug
        suffix = extract_url_identifier(url)
        if slug in used_slugs and suffix:
            slug = f"{base_slug}_{suffix}"
        counter = 2
        while slug in used_slugs:
            slug = f"{base_slug}_{counter}"
            counter += 1
        used_slugs.add(slug)
        entries.append(OfferEntry(title, url, slug, category=category))
    return entries


def parse_markdown_entries(list_path: Path) -> list[dict[str, Any]]:
    entries: list[dict[str, Any]] = []
    category: str | None = None
    for line in list_path.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            category = line[3:].strip()
            continue
        match = ENTRY_RE.match(line.strip())
        if match:
            title, url = match.groups()
            entries.append(
                {
                    "title": title.strip(),
                    "url": url.strip(),
                    "category": category,
                }
            )
    return entries


def parse_json_entries(input_path: Path) -> list[dict[str, Any]]:
    payload = json.loads(input_path.read_text(encoding="utf-8"))
    if isinstance(payload, dict):
        records = payload.get("entries", [])
    else:
        records = payload
    if not isinstance(records, list):
        raise ValueError(f"Format JSON invalide dans {input_path}")

    entries: list[dict[str, Any]] = []
    for item in records:
        if not isinstance(item, dict):
            raise ValueError(f"Entrée JSON invalide dans {input_path}: {item!r}")
        title = item.get("title") or item.get("list_title")
        url = item.get("url")
        if not title or not url:
            raise ValueError(f"Entrée JSON incomplète dans {input_path}: {item!r}")
        entries.append(
            {
                "title": title,
                "url": url,
                "category": item.get("category"),
                "slug": item.get("slug"),
            }
        )
    return entries


def load_entries(input_path: Path, input_format: str) -> list[OfferEntry]:
    if input_format == "markdown":
        raw_entries = parse_markdown_entries(input_path)
    elif input_format == "json":
        raw_entries = parse_json_entries(input_path)
    else:
        raise ValueError(f"Format d'entrée non supporté: {input_format}")
    return build_entries(raw_entries)


def nullable_path(value: Any) -> Path | None:
    if value is None:
        return None
    if isinstance(value, str) and value.strip().lower() in {"", "none", "null", "false", "off"}:
        return None
    return Path(str(value))


def extract_url_identifier(url: str) -> str | None:
    path = urlparse(url).path.rstrip("/")
    match = re.search(r"(JR-\d+|JOBREQ_\d+|R\d+|\d{5,})$", path, flags=re.IGNORECASE)
    if match:
        return slugify(match.group(1))
    return None


def load_legacy_by_url(legacy_dir: Path) -> dict[str, Path]:
    legacy_map: dict[str, Path] = {}
    if not legacy_dir.exists():
        return legacy_map
    for md_path in legacy_dir.glob("*.md"):
        text = md_path.read_text(encoding="utf-8")
        match = re.search(r"\*\*URL:\*\* \[(https?://[^\]]+)\]", text)
        if match:
            legacy_map[normalize_url(match.group(1))] = md_path
    return legacy_map


def normalize_url(url: str) -> str:
    return url.rstrip("/")


def fetch_html(
    session: Any,
    url: str,
    timeout: int,
    retries: int,
) -> tuple[int, str, str]:
    last_error: Exception | None = None
    for attempt in range(1, retries + 1):
        try:
            response = curl_requests.get(
                url,
                headers=REQUEST_HEADERS,
                impersonate="chrome136",
                timeout=timeout,
                allow_redirects=True,
            )
            html = response.text
            if response.status_code >= 400:
                raise RuntimeError(f"HTTP {response.status_code}")
            if is_challenge_page(html):
                raise RuntimeError("Challenge Cloudflare détecté au lieu du vrai HTML.")
            return response.status_code, str(response.url), html
        except Exception as exc:  # noqa: BLE001
            last_error = exc
            if attempt < retries:
                time.sleep(1.5 * attempt)
    raise RuntimeError(f"Echec pour {url}: {last_error}") from last_error


def fetch_with_alternative_source(
    session: Any,
    url: str,
    timeout: int,
    retries: int,
) -> tuple[int, str, str, str]:
    try:
        status_code, final_url, html = fetch_html(session, url, timeout, retries)
        return status_code, final_url, html, url
    except Exception as primary_error:  # noqa: BLE001
        alternative_url = ALTERNATIVE_SOURCE_BY_URL.get(normalize_url(url))
        if not alternative_url:
            raise
        status_code, final_url, html = fetch_html(session, alternative_url, timeout, retries)
        return status_code, final_url, html, alternative_url


def is_challenge_page(html: str) -> bool:
    lowered = html.lower()
    markers = [
        "attention required! | cloudflare",
        "just a moment...",
        "jobteaser | security checkup",
        "sorry, you have been blocked",
        "__cf_chl_tk",
    ]
    return any(marker in lowered for marker in markers)


def select_relevant_container(soup: BeautifulSoup, url: str):
    host = urlparse(url).netloc.lower()
    selectors = DOMAIN_SELECTORS.get(host, []) + GENERIC_SELECTORS
    for selector in selectors:
        container = soup.select_one(selector)
        if container is not None and container.get_text(" ", strip=True):
            return container
    return soup.body or soup


def cleanup_container(container) -> None:
    for tag_name in TAG_NAMES_TO_DROP:
        for tag in container.find_all(tag_name):
            tag.decompose()
    for tag in container.find_all(attrs={"aria-hidden": "true"}):
        # Ces blocs sont majoritairement décoratifs; on préfère le contenu lisible.
        if not tag.find(["p", "li", "h1", "h2", "h3", "h4", "h5", "h6"]):
            tag.decompose()


def extract_page_title(soup: BeautifulSoup, list_title: str) -> str:
    h1 = soup.find("h1")
    if h1:
        title = normalize_inline_text(h1.get_text(" ", strip=True))
        if title:
            return title

    if soup.title and soup.title.string:
        title = normalize_inline_text(soup.title.string)
        title = re.sub(r"\s*[\|\-–]\s*(Safran|JobTeaser|MBDA|Thales|Siemens|Workday).*$", "", title)
        if title:
            return title

    return list_title


def normalize_inline_text(text: str) -> str:
    text = text.replace("\xa0", " ")
    text = re.sub(r"\s+", " ", text)
    return text.strip()


def html_fragment_to_markdown(container) -> str:
    markdown = html_to_markdown(
        str(container),
        heading_style="ATX",
        bullets="-",
    )
    markdown = markdown.replace("\xa0", " ")
    markdown = re.sub(r"\r\n?", "\n", markdown)
    markdown = re.sub(r"[ \t]+\n", "\n", markdown)
    markdown = re.sub(r"\n{3,}", "\n\n", markdown)
    return markdown.strip()


def clone_fragment(fragment):
    return BeautifulSoup(str(fragment), "html.parser")


def extract_wttj_markdown(soup: BeautifulSoup) -> str | None:
    blocks = []
    selectors = [
        '[data-testid="job-metadata-block"]',
        '[data-testid="job-section-description"]',
        '[data-testid="perks_and_benefits_block"]',
        '[data-testid="jobs-section-faq"]',
        '[data-testid="job-section-discover"]',
    ]
    for selector in selectors:
        block = soup.select_one(selector)
        if block is not None and block.get_text(" ", strip=True):
            blocks.append(block)

    for heading_text in ["Le lieu de travail", "Engagements"]:
        heading = soup.find(
            lambda tag: tag.name in {"h3", "h4"}  # noqa: B023
            and normalize_inline_text(tag.get_text(" ", strip=True)) == heading_text
        )
        if heading is not None and heading.parent is not None:
            blocks.append(heading.parent)

    if not blocks:
        return None

    parts: list[str] = []
    seen = set()
    for block in blocks:
        signature = normalize_inline_text(block.get_text(" ", strip=True))
        if not signature or signature in seen:
            continue
        seen.add(signature)
        fragment = clone_fragment(block)
        cleanup_container(fragment)
        markdown = html_fragment_to_markdown(fragment)
        if markdown:
            parts.append(markdown)
    return "\n\n---\n\n".join(parts) if parts else None


def extract_markdown_body(soup: BeautifulSoup, url: str) -> str:
    host = urlparse(url).netloc.lower()
    if host in {"www.welcometothejungle.com", "welcometothejungle.com"}:
        special = extract_wttj_markdown(soup)
        if special:
            return special

    container = select_relevant_container(soup, url)
    cleanup_container(container)
    return html_fragment_to_markdown(container)


def build_markdown(
    page_title: str,
    list_title: str,
    final_url: str,
    source_url: str,
    fetched_at: str,
    status_code: int,
    html_rel_path: str | None,
    markdown_body: str,
) -> str:
    sections = [
        f"# {page_title}",
        "",
        f"**Titre de la liste :** {list_title}",
        f"**URL:** [{final_url}]({final_url})",
        f"**URL source utilisée pour le scraping:** [{source_url}]({source_url})",
        f"**Date de récupération:** {fetched_at}",
        f"**HTTP status:** {status_code}",
        "",
        "## 100% RAW CONTENT",
        "",
        markdown_body,
        "",
    ]
    if html_rel_path:
        sections.insert(6, f"**Snapshot HTML:** [{html_rel_path}]({html_rel_path})")
    return "\n".join(sections)


def write_text(path: Path, content: str, overwrite: bool) -> None:
    if path.exists() and not overwrite:
        return
    path.write_text(content, encoding="utf-8")


def scrape_entry(
    session: Any,
    entry: OfferEntry,
    output_dir: Path,
    html_dir: Path | None,
    timeout: int,
    retries: int,
    overwrite: bool,
) -> dict[str, object]:
    status_code, final_url, html, source_url = fetch_with_alternative_source(
        session, entry.url, timeout, retries
    )
    html_path: Path | None = None
    if html_dir is not None:
        html_path = html_dir / f"{entry.slug}.html"
        write_text(html_path, html, overwrite=overwrite)

    soup = BeautifulSoup(html, "html.parser")
    page_title = extract_page_title(soup, entry.list_title)
    markdown_body = extract_markdown_body(soup, final_url)

    if page_title and page_title.lower() not in markdown_body[:600].lower():
        markdown_body = f"# {page_title}\n\n{markdown_body}"

    fetched_at = datetime.now(timezone.utc).astimezone().isoformat(timespec="seconds")
    html_rel_path = os.path.relpath(html_path, output_dir) if html_path is not None else None
    markdown = build_markdown(
        page_title=page_title,
        list_title=entry.list_title,
        final_url=final_url,
        source_url=source_url,
        fetched_at=fetched_at,
        status_code=status_code,
        html_rel_path=html_rel_path,
        markdown_body=markdown_body,
    )
    md_path = output_dir / f"{entry.slug}.md"
    write_text(md_path, markdown, overwrite=overwrite)
    return {
        "status": "scraped",
        "url": entry.url,
        "source_url": source_url,
        "final_url": final_url,
        "list_title": entry.list_title,
        "page_title": page_title,
        "slug": entry.slug,
        "http_status": status_code,
        "markdown_path": str(md_path),
        "html_path": str(html_path) if html_path is not None else None,
    }


def copy_legacy_entry(
    entry: OfferEntry,
    legacy_path: Path,
    output_dir: Path,
    overwrite: bool,
) -> dict[str, object]:
    content = legacy_path.read_text(encoding="utf-8")
    md_path = output_dir / f"{entry.slug}.md"
    write_text(md_path, content, overwrite=overwrite)
    return {
        "status": "legacy_fallback",
        "url": entry.url,
        "final_url": entry.url,
        "list_title": entry.list_title,
        "page_title": entry.list_title,
        "slug": entry.slug,
        "http_status": None,
        "markdown_path": str(md_path),
        "html_path": None,
        "legacy_source": str(legacy_path),
    }


def ensure_dirs(paths: Iterable[Path]) -> None:
    for path in paths:
        path.mkdir(parents=True, exist_ok=True)


def load_config(config_path: Path) -> dict[str, Any]:
    if not config_path.exists():
        return {}
    data = yaml.safe_load(config_path.read_text(encoding="utf-8")) or {}
    if not isinstance(data, dict):
        raise ValueError(f"Le YAML {config_path} doit contenir un objet à la racine.")
    return data


def resolve_settings(args: argparse.Namespace) -> dict[str, Any]:
    config_path = Path(args.config)
    config = load_config(config_path)
    input_config = config.get("input", {}) if isinstance(config.get("input", {}), dict) else {}

    input_format = args.input_format or input_config.get("format") or "markdown"
    input_path_value = args.input_path or args.list or input_config.get("path") or DEFAULT_LIST_PATH
    output_dir_value = args.output_dir or config.get("output_dir") or DEFAULT_OUTPUT_DIR
    html_dir_value = args.html_dir or config.get("html_dir") or DEFAULT_HTML_DIR
    report_value = args.report or config.get("report") or DEFAULT_REPORT_PATH
    timeout_value = args.timeout if args.timeout is not None else config.get("timeout", 90)
    retries_value = args.retries if args.retries is not None else config.get("retries", 3)
    overwrite_value = args.overwrite if args.overwrite is not None else bool(config.get("overwrite", False))

    output_dir = Path(output_dir_value)
    legacy_dir_value = args.legacy_dir or config.get("legacy_dir") or str(output_dir / "offres")

    return {
        "config_path": config_path,
        "input_format": input_format,
        "input_path": Path(input_path_value),
        "output_dir": output_dir,
        "html_dir": nullable_path(html_dir_value),
        "report_path": nullable_path(report_value),
        "legacy_dir": Path(legacy_dir_value),
        "timeout": int(timeout_value),
        "retries": int(retries_value),
        "overwrite": overwrite_value,
    }


def main() -> int:
    args = parse_args()
    settings = resolve_settings(args)
    input_path = settings["input_path"]
    input_format = settings["input_format"]
    output_dir = settings["output_dir"]
    html_dir = settings["html_dir"]
    report_path = settings["report_path"]
    legacy_dir = settings["legacy_dir"]

    dirs_to_create = [output_dir]
    if html_dir is not None:
        dirs_to_create.append(html_dir)
    ensure_dirs(dirs_to_create)
    entries = load_entries(input_path, input_format)
    legacy_by_url = load_legacy_by_url(legacy_dir)
    report: list[dict[str, object]] = []

    for index, entry in enumerate(entries, start=1):
        print(f"[{index:02d}/{len(entries)}] {entry.list_title}")
        try:
            item = scrape_entry(
                session=None,
                entry=entry,
                output_dir=output_dir,
                html_dir=html_dir,
                timeout=settings["timeout"],
                retries=settings["retries"],
                overwrite=settings["overwrite"],
            )
        except Exception as exc:  # noqa: BLE001
            legacy_path = legacy_by_url.get(normalize_url(entry.url))
            if legacy_path is None:
                print(f"  -> erreur: {exc}")
                report.append(
                    {
                        "status": "failed",
                        "url": entry.url,
                        "list_title": entry.list_title,
                        "slug": entry.slug,
                        "error": str(exc),
                    }
                )
                continue
            print(f"  -> fallback legacy: {legacy_path.name}")
            item = copy_legacy_entry(
                entry=entry,
                legacy_path=legacy_path,
                output_dir=output_dir,
                overwrite=settings["overwrite"],
            )
        else:
            print(f"  -> ok: {Path(item['markdown_path']).name}")
        report.append(item)

    if report_path is not None:
        report_path.write_text(
            json.dumps(
                {
                    "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
                    "config_path": str(settings["config_path"]),
                    "input_format": input_format,
                    "input_path": str(input_path),
                    "output_dir": str(output_dir),
                    "html_dir": str(html_dir) if html_dir is not None else None,
                    "entries": report,
                },
                ensure_ascii=False,
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )

    failed = [entry for entry in report if entry["status"] == "failed"]
    print()
    print(f"Fichiers traités: {len(report)}/{len(entries)}")
    print(f"Echecs: {len(failed)}")
    print(f"Rapport: {report_path if report_path is not None else '(désactivé)'}")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
