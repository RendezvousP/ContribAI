# ContribAI 🤖

> **AI Agent that automatically contributes to open source projects on GitHub**

ContribAI discovers open source repositories, analyzes them for improvement opportunities, generates high-quality fixes, and submits them as Pull Requests — all autonomously.

[![Python 3.11+](https://img.shields.io/badge/python-3.11+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-169%20passed-brightgreen)](#)
[![Version](https://img.shields.io/badge/version-0.3.0-blue)](#)

---

## ✨ Features

### Core Pipeline
- 🔍 **Smart Discovery** – Finds contribution-friendly repos by language, stars, activity, and community health
- 🔒 **Security Analysis** – Detects hardcoded secrets, SQL injection, XSS, and more
- ✨ **Code Quality** – Finds dead code, missing error handling, complexity issues
- 📝 **Documentation** – Catches missing docstrings, incomplete READMEs, broken links
- 🎨 **UI/UX** – Identifies accessibility issues, missing loading states, responsive design gaps
- 🤖 **Multi-LLM** – Supports Gemini (primary), OpenAI, Anthropic, and local models via Ollama
- 📤 **Auto-PR** – Forks, branches, commits, and creates PRs with detailed descriptions
- 🧠 **Memory** – Tracks analyzed repos and submitted PRs to avoid duplicate work

### Intelligence (v0.3.0)
- 🎯 **Issue Solver** – Reads open GitHub issues, classifies them, and generates targeted fixes
- 🧩 **Framework Detection** – Auto-detects Django, Flask, FastAPI, React/Next.js, Express with tailored analysis
- 📊 **Quality Scorer** – 7-check quality gate before PR submission (prevents low-quality PRs)
- 🔄 **Retry & Resilience** – Exponential backoff with jitter for GitHub API and LLM calls
- ⚡ **Response Caching** – LRU cache for API responses to reduce costs

## 🏗️ Architecture

```
Discovery → Analysis → Generation → Quality Gate → PR
    │           │           │            │            │
    ▼           ▼           ▼            ▼            ▼
 GitHub    4 Analyzers   LLM-based    7-check      Fork+Branch
 Search    + Framework   code gen     scorer       +Commit+PR
           Strategies   + self-review
```

## 📦 Installation

```bash
git clone https://github.com/tang-vu/ContribAI.git
cd ContribAI
pip install -e ".[dev]"
```

## ⚙️ Configuration

```bash
cp config.example.yaml config.yaml
```

Edit `config.yaml`:

```yaml
github:
  token: "ghp_your_token_here"

llm:
  provider: "gemini"              # gemini | openai | anthropic | ollama
  model: "gemini-2.5-flash"
  api_key: "your_api_key"

discovery:
  languages: [python, javascript]
  stars_range: [100, 5000]
```

## 🚀 Usage

### Auto-discover and contribute

```bash
contribai run                                       # Full autonomous run
contribai run --dry-run                             # Preview without creating PRs
contribai run --language python --stars 100-3000    # Filter by language & stars
contribai run --max-prs 3                           # Limit PRs
```

### Target a specific repo

```bash
contribai target https://github.com/owner/repo
contribai target https://github.com/owner/repo --types security_fix,docs_improve
contribai target https://github.com/owner/repo --dry-run
```

### Solve open issues

```bash
contribai solve https://github.com/owner/repo               # Solve solvable issues
contribai solve https://github.com/owner/repo --max-issues 3 # Limit issues
contribai solve https://github.com/owner/repo --dry-run      # Preview only
```

### Analyze only

```bash
contribai analyze https://github.com/owner/repo
```

### Status & stats

```bash
contribai status        # Check submitted PRs
contribai stats         # Overall statistics
contribai config        # View current config
```

## 🧩 Supported Frameworks

| Framework | Detection | Analysis Focus |
|-----------|-----------|---------------|
| **Django** | `manage.py`, `settings.py` | CSRF, N+1 queries, migrations, admin |
| **Flask** | `from flask` imports | Blueprints, factory pattern, CSRF |
| **FastAPI** | `from fastapi` imports | Async, auth dependencies, OpenAPI |
| **React/Next.js** | `package.json`, `.tsx` | XSS, memo, accessibility, SSR |
| **Express** | `package.json`, `require('express')` | Helmet, rate limiting, validation |

## 📁 Project Structure

```
contribai/
├── core/              # Config, models, exceptions, retry utilities
├── llm/               # Multi-provider LLM layer (Gemini, OpenAI, Anthropic, Ollama)
├── github/            # GitHub API client & repo discovery
├── analysis/          # LLM-powered code analysis + framework strategies
├── generator/         # Contribution generator + self-review + quality scorer
├── issues/            # Issue-driven contribution solver
├── pr/                # PR lifecycle manager (fork → branch → commit → PR)
├── orchestrator/      # Pipeline orchestrator & persistent memory
└── cli/               # Rich CLI interface (7 commands)

.agents/
├── agents/            # 8 team agent roles (Tech Lead, Backend Dev, Security, QA, DevOps, Writer, Reviewer, PM)
└── workflows/         # 10 dev workflows (/dev, /review, /release, /test, /security-audit, /docs, /git-flow, /setup, /debug, /deploy)

.github/
├── workflows/         # CI (lint+test+security) & Release pipelines
├── ISSUE_TEMPLATE/    # Bug, feature, security report templates
└── pull_request_template.md
```

## 🧪 Testing

```bash
pytest tests/ -v                  # Run all 169 tests
pytest tests/ -v --cov=contribai  # With coverage
ruff check contribai/             # Lint
ruff format contribai/            # Format
```

## 🛡️ Safety

- **Daily PR limit** – Configurable max PRs per day (default: 10)
- **Quality scorer** – 7-check gate prevents low-quality PRs
- **Self-review** – LLM reviews its own generated code before PR
- **Dry run mode** – Preview everything without creating actual PRs
- **Rate limit awareness** – Exponential backoff with jitter
- **Memory** – Never analyzes the same repo twice in a session

## 📄 License

MIT License – see [LICENSE](LICENSE) for details.

---

**Made with ❤️ for the open source community**
