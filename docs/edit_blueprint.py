import sys
import re

with open("blueprint.md", "r") as f:
    text = f.read()

# 1. Fix LlmClient and Embedder
old_llm_trait = """    /// Embeddings (peut être un autre service, ex: Voyage).
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError>;

    fn name(&self) -> &'static str;"""

new_llm_trait = """    fn name(&self) -> &'static str;
}

#[async_trait]
pub trait Embedder: Send + Sync {
    /// Embeddings (peut être un autre service, ex: Voyage).
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError>;"""

text = text.replace(old_llm_trait, new_llm_trait)

# 2. Delete 17.4
phase_pattern = re.compile(r"### 17\.4\. Phasage Recommandé.*?### 6\.2\. Transition pragmatique", re.DOTALL)
text = re.sub(phase_pattern, "### 6.2. Transition pragmatique", text)

# 3. Delete Logs Claude
logs_claude_pattern = re.compile(r"### Construction Initiale du Squelette \(Logs Claude\).*?(?=### README - Backend Rust)", re.DOTALL)
text = re.sub(logs_claude_pattern, "", text)

# 4. Add the 3 structured deliverables to "Contrat des livrables"
old_contrat = """### Contrat des livrables

Le point clé du système est la stabilité des schémas.

- la structure JSON du `resume` ne change pas
- la structure JSON de la `cover_letter` ne change pas
- seule la matière générée change

Cela permet :

- de garder un renderer HTML stable
- d'ajouter une future édition pilotée par IA
- de cibler une section précise plus tard sans régénérer tout le document
"""

new_contrat = old_contrat + """
#### Schémas Rust des 3 Livrables

Voici les contrats JSON stricts que le LLM doit respecter et qui sont validés à la compilation :

```rust
// 1. Restitution (Fiche de synthèse)
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FitAnalysis {
    pub synthese: String,
    pub fit_score: u8, // 0-100
    pub points_forts: Vec<String>,
    pub points_faibles: Vec<String>,
    pub questions_entretien: Vec<String>,
}

// 2. Resume (CV)
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Resume {
    pub nom: String,
    pub titre_cible: String,
    pub accroche: String,
    pub experiences: Vec<Experience>,
    pub competences: Vec<CompetenceGroup>,
}

// 3. Cover Letter (Lettre)
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CoverLetter {
    pub date: String,
    pub entreprise: String,
    pub paragraphes: Vec<Paragraphe>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Paragraphe {
    pub role: ParagrapheRole, // Intro, Fit, Apport, Conclusion
    pub contenu: String,
}
```
"""
text = text.replace(old_contrat, new_contrat)

# 5. Move "Quand abandonner Rust" from Section 1 to Section 9 (Annexes)
abandonner_pattern = re.compile(r"### Quand abandonner Rust.*?(?=## 2\. Architecture Backend & Stack Technique)", re.DOTALL)
abandonner_match = abandonner_pattern.search(text)
if abandonner_match:
    abandonner_text = abandonner_match.group(0)
    text = text.replace(abandonner_text, "")
    text += "\n### Critères de pivot (Quand abandonner Rust)\n\n" + abandonner_text.replace("### Quand abandonner Rust\n", "")

with open("blueprint.md", "w") as f:
    f.write(text)
print("done")
