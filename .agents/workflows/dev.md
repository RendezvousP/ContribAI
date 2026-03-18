---
description: Development workflow – code, lint, test, commit cycle for implementing features and fixes
---

# Development Workflow

## Steps

1. **Pull latest changes**
// turbo
```bash
git pull origin main
```

2. **Create feature branch**
```bash
git checkout -b <type>/<short-description>
```
Types: `feat/`, `fix/`, `refactor/`, `docs/`, `perf/`, `test/`, `chore/`

3. **Write code changes**
Follow the Backend Developer agent standards:
- Async-first for I/O
- Type hints on all public APIs
- Pydantic models for data
- Custom exceptions from `contribai.core.exceptions`
- Logging via `logging.getLogger(__name__)`

4. **Write tests alongside code**
```bash
# Create test file in tests/unit/ matching the module
# e.g., contribai/analysis/analyzer.py → tests/unit/test_analyzer.py
```

5. **Run lint**
// turbo
```bash
ruff check contribai/ --fix
```

6. **Run format**
// turbo
```bash
ruff format contribai/ tests/
```

7. **Run tests**
// turbo
```bash
pytest tests/ -v --tb=short
```

8. **Check coverage**
// turbo
```bash
pytest tests/ --cov=contribai --cov-report=term-missing
```

9. **Stage changes**
```bash
git add -A
```

10. **Commit with conventional message**
```bash
git commit -m "<type>: <short description>"
```
Format: `type: description` where type is one of:
- `feat` – New feature
- `fix` – Bug fix
- `refactor` – Code restructuring
- `docs` – Documentation
- `test` – Tests
- `perf` – Performance improvement
- `chore` – Maintenance

11. **Push to remote**
```bash
git push origin <branch-name>
```

12. **Create Pull Request**
Open PR on GitHub with the PR template, request review from Code Reviewer agent.

## Patch Release Convention

After committing small fixes or improvements directly to `main` (without a feature branch), do a **patch release** to keep versions trackable:

### When to Patch Release (0.x.Z)
- Bug fixes (search/replace, checkbox ticking, etc.)
- Small improvements (auto-issue creation, compliance loop)
- Documentation fixes
- Config changes

### When to Minor Release (0.Y.0)
- New major features (multi-model routing, repo guidelines compliance)
- New commands or CLI options
- New analyzers or generators

### Quick Patch Release Steps
1. Bump version in `contribai/__init__.py`: `__version__ = "0.x.Z"`
2. Commit: `git commit -am "chore: release v0.x.Z"`
3. Tag: `git tag -a v0.x.Z -m "Release v0.x.Z"`
4. Push: `git push origin main --tags`

### Rule of Thumb
- **1-3 fix commits** since last release → patch release
- **Major feature complete** → minor release
- Accumulating too many unreleased commits is bad — release often!

