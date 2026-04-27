<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="gmail/assets/n8n-logo.png" alt="n8n" width="120">
  <img src="multi-scraper/assets/make-logo.png" alt="Make" width="160">
  <br /><br />
  <img src="https://img.shields.io/badge/n8n-EA4B71?style=for-the-badge&logo=n8n&logoColor=white" alt="n8n">
  <img src="https://img.shields.io/badge/Make-6D00CC?style=for-the-badge&logo=integromat&logoColor=white" alt="Make">
  <img src="https://img.shields.io/badge/OpenAI-412991?style=for-the-badge&logo=openai&logoColor=white" alt="OpenAI">
  <img src="https://img.shields.io/badge/Google_Gemini-4285F4?style=for-the-badge&logo=google&logoColor=white" alt="Gemini">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
</div>
<!-- markdownlint-enable MD033 -->

---

This repository bundles **three automation workflows** (n8n, Make) with AI integration (OpenAI, Gemini), deployable standalone or via Docker. Each subfolder contains the exported workflow, assets and, when applicable, a frontend or demo.

### Repository structure

```
no-low-code/
├── gmail/                  # Gmail AI Dashboard (n8n + Docker + frontend)
│   ├── docker-compose.yml
│   ├── json/workflow.json
│   ├── assets/
│   └── frontend/
├── multi-scraper/          # Multi-Scraper (Make → Google Sheets)
│   ├── json/workflow.json
│   └── assets/
└── tiktok/                 # TikTok Intelligence (n8n → Airtable)
    ├── json/workflow.json
    └── assets/
```

### Technical Core

| Layer | Stack |
|-------|--------|
| **Orchestration** | n8n, Make |
| **AI** | OpenAI GPT-3.5, Google Gemini |
| **Scraping** | Apify (TikTok, Instagram) |
| **Storage** | Airtable, Google Sheets, JSON (file) |
| **APIs** | Gmail API, TikTok (via Apify) |
| **Runtime** | Docker (Gmail), Make/n8n cloud (others) |

### Global architecture

**Research context** : This repository is used for research on **how to collect and use data from social networks and communication channels** (email, TikTok, Instagram, RSS) **through no-code / low-code automation tools**. The goal is to assess orchestration (n8n, Make), scraping (Apify), AI enrichment (LLM, vision) and storage (Airtable, Google Sheets) to build reproducible pipelines without heavy custom development.

The three workflows share a single high-level pattern: **data sources → orchestration → AI enrichment → storage or delivery**.

```mermaid
flowchart LR
    A[Data sources] --> B[Orchestration]
    B --> C[AI enrichment]
    C --> D[Storage / delivery]
```

---

## Workflows

### [Gmail AI Dashboard](gmail/)

End-to-end pipeline: fetch Gmail via the official API, analyse with OpenAI (summaries, urgency detection), then serve results in a **web interface** (sort, pin, archive, filters). One-command deploy with **Docker**. Ideal for centralising email monitoring and prioritising messages without opening Gmail.

```
gmail/
├── docker-compose.yml
├── json/workflow.json
├── assets/
└── frontend/
```

**Install (this workflow only)**

```bash
git clone --filter=blob:none --sparse https://github.com/RomeoCavazza/no-low-code.git
cd no-low-code && git sparse-checkout set gmail && cd gmail
```

| Role | Details |
|------|--------|
| Extraction | Automatic fetch of latest emails (Gmail API, 24h window) |
| Analysis | Summaries + urgency detection (OpenAI GPT-3.5) |
| UI | Vanilla JS dashboard (HTML5, CSS3, localStorage, Lucide) |
| Deploy | `docker-compose` (n8n + static server) |

![Gmail Workflow](gmail/assets/n8n-workflow.png)

*n8n workflow: trigger, Gmail fetch, OpenAI analysis, write JSON.*

![Gmail Frontend](gmail/assets/front-page.png)

*Web dashboard: AI summary, urgency badge, filters and email actions.*

---

### [Multi-Scraper IA](multi-scraper/)

Multi-source automated monitoring: aggregate **RSS feeds** (NVIDIA, OpenAI, Google, Microsoft…) and **Instagram** tech accounts via Apify, enrich with GPT summaries and Gemini image analysis, **deduplicate**, then export to **Google Sheets**. Run an AI monitoring dashboard with no code. **Demo** : [Google Sheet](https://docs.google.com/spreadsheets/d/17JXOTxNk7-EDYpSQIKgBH-hyClpwn7jkmSknl3Azs1A/edit).

```
multi-scraper/
├── json/workflow.json
└── assets/
```

**Install (this workflow only)**

```bash
git clone --filter=blob:none --sparse https://github.com/RomeoCavazza/no-low-code.git
cd no-low-code && git sparse-checkout set multi-scraper && cd multi-scraper
```

| Role | Details |
|------|--------|
| Aggregation | RSS + Instagram (Apify) |
| Enrichment | GPT-3.5 summaries + Gemini Pro image analysis |
| Deduplication | Pre-export processing to avoid duplicates |
| Export | Google Sheets (Title, URL, date, source, AI summary) |

![Make Workflow](multi-scraper/assets/make-workflow.png)

*Make scenario: RSS + Instagram aggregation, OpenAI + Gemini enrichment, deduplication, Google Sheets export.*

![Multi-Scraper Data](multi-scraper/assets/data-sheet.png)

*Google Sheets output: title, URL, date, source, AI summary.*

---

### [TikTok Intelligence](tiktok/)

TikTok extraction by **keywords** or **accounts**: metrics (views, likes, comments, shares), **VTT subtitle** extraction, summaries and insights via OpenAI, then save to **Airtable**. Useful for creator monitoring, trends or video content analysis.

```
tiktok/
├── json/workflow.json
└── assets/
```

**Install (this workflow only)**

```bash
git clone --filter=blob:none --sparse https://github.com/RomeoCavazza/no-low-code.git
cd no-low-code && git sparse-checkout set tiktok && cd tiktok
```

| Role | Details |
|------|--------|
| Source | TikTok via Apify (keywords or handles) |
| Metrics | Views, likes, comments, shares |
| Transcripts | Automatic VTT subtitles |
| Analysis | Summaries and insights OpenAI → Airtable |

![TikTok Workflow](tiktok/assets/n8n-workflow.png)

*n8n workflow: web form, Apify TikTok, VTT + OpenAI, Airtable.*

![TikTok Request form](tiktok/assets/request-form.png)

*Web form: keywords, accounts, period, results.*

![TikTok Data](tiktok/assets/data-table.png)

*Airtable table: video URL, author, metrics, transcript, AI summary.*

---

Each workflow is **self-contained**: clone the subfolder, import the JSON into n8n or Make, set credentials (API keys, OAuth, Apify tokens), and run. No custom backend is required beyond the orchestrator and cloud services (Airtable, Sheets, Gmail).
