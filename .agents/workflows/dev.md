---
description: ContribAI development workflow - code, patrol, hunt, and release
---

# ContribAI Development Workflow

// turbo-all

## Code Changes

1. Make changes to the relevant files in `contribai/`
2. Run formatting and linting:
   ```bash
   ruff format contribai/ tests/
   ruff check contribai/ tests/ --fix
   ```
3. Run tests:
   ```bash
   pytest tests/ -q --tb=short --cov=contribai --cov-fail-under=50
   ```
4. Commit with conventional commits:
   ```bash
   git add -A && git commit -m "feat|fix|refactor: description"
   ```
5. Push and verify CI:
   ```bash
   git push origin main
   ```

## PR Patrol

Run patrol to monitor and respond to PR review feedback:

```bash
# Dry run (no changes)
contribai patrol --dry-run

# Target specific PR
contribai patrol --pr <PR_NUMBER>

# Live run (responds to feedback)
contribai patrol
```

### Key files:
- `contribai/pr/patrol.py` - Patrol engine
- `contribai/core/models.py` - FeedbackItem, PatrolResult, FeedbackAction
- `contribai/github/client.py` - GitHub API (create_or_update_file, get_assigned_issues)
- `contribai/cli/main.py` - CLI patrol command

### DCO Signoff
All commits via GitHub API automatically include `Signed-off-by:` trailer.
Configured via `github.dco_signoff: true` in `config.yaml`.

If a repo requires DCO and existing commits lack signoff:
```bash
git rebase HEAD~N --signoff
git push --force-with-lease origin <branch>
```

### Bot Review Context
When a maintainer replies to a bot review (Coderabbit, etc.), patrol reads the bot's
original analysis via `in_reply_to_id` and passes it as context to the LLM for
generating accurate code fixes.

## Hunt Mode

```bash
# Hunt for repos and generate PRs
contribai hunt --rounds 1 --repos 20

# Dry run
contribai hunt --rounds 1 --repos 5 --dry-run
```

## Release

1. Bump version in `contribai/__init__.py` and `pyproject.toml`
2. Update `CHANGELOG.md` with new version section
3. Commit and push
4. Create release:
   ```bash
   gh release create v<VERSION> --repo tang-vu/ContribAI --title "v<VERSION> - Title" --generate-notes
   ```
5. Verify all CI checks pass

## Config Reference

Key config fields in `config.yaml`:

```yaml
github:
  dco_signoff: true          # Auto Signed-off-by (default: true)
  max_repos_per_run: 5
  max_prs_per_day: 10

llm:
  provider: gemini
  model: gemini-2.5-flash

contribution:
  commit_convention: conventional
```
