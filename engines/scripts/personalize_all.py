#!/usr/bin/env python3
import json
import re
from pathlib import Path

# --- CONFIGURATION ---

MASTERS = {
    "IA": "Master of Science — Intelligence artificielle",
    "DATA": "Master of Science — Big Data",
    "CLOUD": "Master of Science — Cloud",
    "CYBER": "Master of Science — Cybersécurité",
    "IMMERSIVE": "Master of Science — Systèmes immersifs et Réalités virtuelles",
    "ROBOTICS": "Master of Science — Robotique & IoT"
}

PRE_MSC = {
    "DEVOPS": "Pré-MSc — Docker, Jenkins, Ansible, Linux, CI/CD, Rust, Python.",
    "DATA_IA": "Pré-MSc — Python, CUDA, NumPy, Pandas, Rust, Algorithmique.",
    "EMBEDDED": "Pré-MSc — Rust, Linux, Python, Java, Bash, Réseaux.",
    "WEB_DEV": "Pré-MSc — JavaScript, PHP, Java, SQL, Rust, Next.js, Docker."
}

# Mapping: job_id keyword -> (Master_Key, PreMSc_Key)
MAPPING_RULES = [
    (r"ia|intelligence|agent|llm|vision|dsi", ("IA", "DATA_IA")),
    (r"data|analyst|big data|statistique|gallica", ("DATA", "DATA_IA")),
    (r"devops|sre|industrialisation|genia", ("CLOUD", "DEVOPS")),
    (r"cyber", ("CYBER", "DEVOPS")),
    (r"full[- ]stack|web|développeur java|api", ("CLOUD", "WEB_DEV")),
    (r"unreal|ue5|mersif|3d", ("IMMERSIVE", "EMBEDDED")),
    (r"embarqué|électronique|robotique|iot|décision|decision|algorithme|autonome|simu", ("ROBOTICS", "EMBEDDED")),
]

PITCH_TEMPLATES = {
    "IA": "Étudiant en Master Intelligence Artificielle, passionné par l'IA générative, je souhaite développer et mettre en pratique mes compétences en Computer Vision, NLP et IA appliquée au sein de {company}.",
    "DATA": "Étudiant en Master Big Data, passionné par l'ingénierie des données, je souhaite développer et mettre en pratique mes compétences en pipelines ETL, analyse statistique et Cloud au sein de {company}.",
    "CLOUD": "Étudiant en Master Cloud, passionné par les architectures scalables, je souhaite développer et mettre en pratique mes compétences en architecture distribuée, virtualisation et services Cloud au sein de {company}.",
    "DEVOPS": "Étudiant en Master Cloud, passionné par l'automatisation, je souhaite développer et mettre en pratique mes compétences en DevOps, Docker et pipelines CI/CD au sein de {company}.",
    "CYBER": "Étudiant en Master Cybersécurité, passionné par la sécurité offensive, je souhaite développer et mettre en pratique mes compétences en durcissement système, réseaux et audit de sécurité au sein de {company}.",
    "IMMERSIVE": "Étudiant en Master Systèmes Immersifs, passionné par la 3D temps réel, je souhaite développer et mettre en pratique mes compétences en Unreal Engine 5, C++ et simulation au sein de {company}.",
    "ROBOTICS": "Étudiant en Master Robotique & IoT, passionné par les systèmes embarqués, je souhaite développer et mettre en pratique mes compétences en instrumentation, bas niveau et programmation système au sein de {company}."
}

def get_job_categories(job_id: str, title: str):
    full_text = (job_id + " " + title).lower()
    for pattern, keys in MAPPING_RULES:
        if re.search(pattern, full_text):
            return keys
    return ("ROBOTICS", "EMBEDDED")

def personalize_instance(instance_path: Path, company_name: str):
    resume_path = instance_path / "resume.json"
    cl_path = instance_path / "cover-letter.json"
    
    m_key, p_key = get_job_categories(instance_path.name, company_name)
    
    # 1. PERSONALIZE RESUME
    if resume_path.exists():
        with open(resume_path, 'r') as f:
            data = json.load(f)
        
        # Education
        if len(data.get("education", [])) >= 1:
            data["education"][0]["degree"] = MASTERS[m_key]
            # data["education"][1]["degree"] = PRE_MSC[p_key]
        
        # Pitch
        pitch_variant = m_key
        if m_key == "CLOUD" and p_key == "DEVOPS": pitch_variant = "DEVOPS"
        data["profile"]["pitch"] = PITCH_TEMPLATES.get(pitch_variant).format(company=company_name)
        
        # Location & Durations (Enforce standards)
        data["profile"]["location"] = "Paris, 11e"
        data["apprenticeship"]["duration"] = "24 mois"
        data["apprenticeship"]["start"] = "septembre 2026"
        
        # Skills Category Rename (ML -> IA)
        for skill in data.get("skills", []):
            if skill.get("category") == "Machine Learning":
                skill["category"] = "Intelligence Artificielle"
        
        with open(resume_path, 'w') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

    # 2. PERSONALIZE COVER LETTER
    if cl_path.exists():
        with open(cl_path, 'r') as f:
            cl_data = json.load(f)
        
        cl_data["letter"]["company"] = company_name
        cl_data["letter"]["date"] = "27 avril 2026"
        
        # Sync Subject with Resume Title
        if resume_path.exists():
            cl_data["letter"]["subject"] = f"ALTERNANCE - {data['profile']['title'].replace('ALTERNANCE - ', '')}"
        
        # Bolding Keyword
        cl_data["letter"]["boldKeyword"] = "ALTERNANCE"
        
        # Remove jargon/apprenti
        def clean_text(t):
            t = re.sub(r"apprenti[e]?|apprentissage", "alternance", t, flags=re.IGNORECASE)
            return t
        
        cl_data["letter"]["subject"] = clean_text(cl_data["letter"]["subject"])
        cl_data["letter"]["paragraphs"] = [clean_text(p) for p in cl_data["letter"]["paragraphs"]]
        
        with open(cl_path, 'w') as f:
            json.dump(cl_data, f, indent=2, ensure_ascii=False)

def main():
    root = Path(__file__).parent.parent.parent
    instances_dir = root / "engines" / "data" / "offres" / "json"
    liste_path = root / "engines" / "data" / "offres" / "liste.json"

    if not liste_path.exists():
        print("Error: liste.json not found.")
        return

    with open(liste_path, 'r') as f:
        offres = json.load(f)["entries"]
    
    company_map = {o["job_id"]: o["title"].split(" - ")[0] for o in offres}

    print("️  Syncing and Personalizing all instances...")
    count = 0
    for instance_path in instances_dir.iterdir():
        if not instance_path.is_dir(): continue
        job_id = instance_path.name
        company = company_map.get(job_id, "votre entreprise")
        personalize_instance(instance_path, company)
        print(f"{job_id} synchronized.")
        count += 1
    
    print(f"\nDone. {count} instances personalized.")

if __name__ == "__main__":
    main()
