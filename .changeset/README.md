# Changesets

This directory contains changeset files that describe changes to the project.

## What are changesets?

Changesets are a way to manage versioning and changelogs. Each changeset file describes a change and the type of version bump it requires (patch, minor, or major).

## Creating a changeset

### Using the helper script (recommended)

```bash
./scripts/add-changeset.sh
```

This will interactively prompt you for:

- The type of change (patch, minor, or major)
- A description of the change

### Manual creation

Create a new file in `.changeset/` with a unique name (e.g., `.changeset/cool-feature-123.md`):

```markdown
---
'tasuku-rs': patch
---

Fixed a bug in task rendering
```

## Changeset format

Each changeset file follows this format:

```markdown
---
'tasuku-rs': <bump-type>
---

<description of the change>
```

Where `<bump-type>` is one of:

- `patch` - Bug fixes and minor changes (0.1.0 → 0.1.1)
- `minor` - New features (0.1.0 → 0.2.0)
- `major` - Breaking changes (0.1.0 → 1.0.0)

## Examples

### Patch (bug fix)

```markdown
---
'tasuku-rs': patch
---

Fixed terminal rendering issue on Windows
```

### Minor (new feature)

```markdown
---
'tasuku-rs': minor
---

Added support for custom task icons
```

### Major (breaking change)

```markdown
---
'tasuku-rs': major
---

Changed TaskApi interface to use builder pattern. This is a breaking change that requires updating existing code.
```

## Workflow

1. **Make your changes** to the codebase
2. **Create a changeset** describing your changes
3. **Commit both** your code changes and the changeset file
4. **Open a PR** - the changeset will be reviewed along with your code
5. **After merge** - A GitHub Action will automatically:
   - Process all changesets
   - Update the version in `Cargo.toml`
   - Generate/update `CHANGELOG.md`
   - Create a PR with these changes
6. **Merge the version PR** - This will trigger the publish workflow

## Tips

- One changeset per logical change
- Be descriptive in your changeset messages - they become changelog entries
- If you're unsure about the bump type, ask in your PR
- Multiple changesets in one PR are fine
- You can have changesets without code changes (e.g., documentation updates)
