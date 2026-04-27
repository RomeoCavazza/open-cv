# Prompt — Context Engineering (DRYVIA)

Ce fichier est le **prompt maître** pour générer la base de connaissances markdown du projet DRYVIA (branding, design system, product data, project context). À utiliser en amont du vibecoding : une fois les 4 fichiers produits, ils servent d’annexes pour tous les prompts suivants.

---

## Workflow recommandé

```mermaid
flowchart LR
    A[Brief projet] --> B[Prompt Context Engineering]
    B --> C[Agent]
    C --> D[4 fichiers .md]
    D --> E[brand_guidelines design_system product_data project_context]
```

| Étape | Action |
|-------|--------|
| 1 | Préparer un **brief projet** (niche, cible, stack, positionnement). |
| 2 | Coller le brief + ce prompt dans une nouvelle conversation. |
| 3 | Vérifier que les 4 fichiers sont bien créés sans placeholder (Lorem, etc.). |
| 4 | Placer les fichiers dans `docs/IV. Context Engineering/Contexte/MarkDowns/`. |

---

## Bloc prompt (copier-coller)

<Context>
You are a Technical Documentation Architect specializing in creating comprehensive project documentation for modern full-stack e-commerce applications.
</Context>

<Role>
Senior Technical Writer & Project Scaffolding Specialist
</Role>

<Action>
Create a set of markdown (.md) files that will serve as the single source of truth for a development team building the "DRYVIA" premium indoor shoe brand website. These files must contain all branding guidelines, design systems, product data, and technical specifications needed to build the project.
</Action>

<Constraints>
- Create exactly 4 markdown files with the specified names and content.
- All content must be pulled directly from the provided project brief above.
- Use proper markdown formatting with headers, tables, code blocks where appropriate.
- Do not add any placeholder text (like "Lorem Ipsum") - use only real data from the brief.
- Ensure all citations/references from the brief are properly included as comments or references.
</Constraints>

<Format>
Create four separate markdown files with the following structure:
1. `brand_guidelines.md` - Brand identity, verbal identity, positioning
2. `design_system.md` - Colors, typography, UI components
3. `product_data.md` - Product specifications, features, pricing, images
4. `project_context.md` - Technical stack, architecture rules, workflow
</Format>

<Tone>
Professional, precise, technical, and instructional. This documentation should be ready for developers to use immediately.
</Tone>

<Instructions>
Extract and organize all information from the provided project brief into the four markdown files as follows:

1. **brand_guidelines.md**:
   - Include Brand Name and strategic rationale
   - Brand Identity (Vibe, Positioning, Tone)
   - Verbal Identity (Taglines)
   - The Niche description
   - Target Audience
   - Problem Statement
   - Solution overview
   - Unique Selling Proposition table

2. **design_system.md**:
   - Brand Identity keywords
   - Complete Color Palette table with hex codes and usage descriptions
   - Typography specifications
   - Detailed UI Components (Buttons, Cards, Images)
   - Include all specific color hex codes and styling rules mentioned

3. **product_data.md**:
   - The flagship model "DRYVIA One" details
   - Price and category
   - Key Features with their USPs
   - Product description
   - Image mapping table linking image names to their purpose
   - Ensure all feature citations are included as comments

4. **project_context.md**:
   - Project Rules & Context statement
   - Complete Tech Stack specification
   - Architecture Guidelines (Frontend/Backend separation)
   - Frontend Rules (Next.js specifics)
   - Backend Rules (TypeScript patterns)
   - Workflow instructions for developers

Each file should be self-contained and reference the others where necessary. Use proper markdown formatting including headers, tables, bullet points, and code blocks for technical specifications.
</Instructions>