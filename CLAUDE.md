
# Project Instructions

**This project is managed through AgilePlus.**

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
- CLI: cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>

## Work Requirements

1. Check for AgilePlus spec before implementing
2. Create spec for new work: agileplus specify --title "<feature>" --description "<desc>"
3. Update work package status: agileplus status <feature-id> --wp <wp-id> --state <state>
4. No code without corresponding AgilePlus spec

## Branch Discipline

- Feature branches in repos/worktrees/<project>/<category>/<branch>
- Canonical repository tracks main only
- Return to main for merge/integration checkpoints

## UTF-8 Encoding

All markdown files must use UTF-8. Validate with:
```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

## AgilePlus Reference

- Specs: AgilePlus/kitty-specs/<feature-id>/
- Docs: AgilePlus/docs/
- Workflows: AgilePlus/docs/workflow/
- Worklog: AgilePlus/.work-audit/worklog.md

