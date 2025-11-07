## Getting started

Looking for something specific? Jump ahead:

- [Tips & shortcuts](#tips--shortcuts) – hotkeys, resume flow, prompts
- [Non-interactive runs](./exec.md) – automate with `cass exec`
- Ready for deeper customization? Head to [`advanced.md`](./advanced.md)

### CLI usage

| Command            | Purpose                            | Example                         |
| ------------------ | ---------------------------------- | ------------------------------- |
| `cass`            | Interactive TUI                    | `cass`                         |
| `cass "..."`      | Initial prompt for interactive TUI | `cass "fix lint errors"`       |
| `cass exec "..."` | Non-interactive "automation mode"  | `cass exec "explain utils.ts"` |

Key flags: `--model/-m`, `--ask-for-approval/-a`.

### Resuming interactive sessions

- Run `cass resume` to display the session picker UI
- Resume most recent: `cass resume --last`
- Resume by id: `cass resume <SESSION_ID>` (You can get session ids from /status or `~/.cass/sessions/`)

Examples:

```shell
# Open a picker of recent sessions
cass resume

# Resume the most recent session
cass resume --last

# Resume a specific session by id
cass resume 7f9f9a2e-1b3c-4c7a-9b0e-123456789abc
```

### Running with a prompt as input

You can also run Cass CLI with a prompt as input:

```shell
cass "explain this codebase to me"
```

### Example prompts

Below are a few bite-size examples you can copy-paste. Replace the text in quotes with your own task.

| ✨  | What you type                                                                   | What happens                                                               |
| --- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1   | `cass "Refactor the Dashboard component to React Hooks"`                       | Cass rewrites the class component, runs `npm test`, and shows the diff.   |
| 2   | `cass "Generate SQL migrations for adding a users table"`                      | Infers your ORM, creates migration files, and runs them in a sandboxed DB. |
| 3   | `cass "Write unit tests for utils/date.ts"`                                    | Generates tests, executes them, and iterates until they pass.              |
| 4   | `cass "Bulk-rename *.jpeg -> *.jpg with git mv"`                               | Safely renames files and updates imports/usages.                           |
| 5   | `cass "Explain what this regex does: ^(?=.*[A-Z]).{8,}$"`                      | Outputs a step-by-step human explanation.                                  |
| 6   | `cass "Carefully review this repo, and propose 3 high impact well-scoped PRs"` | Suggests impactful PRs in the current codebase.                            |
| 7   | `cass "Look for vulnerabilities and create a security review report"`          | Finds and explains security bugs.                                          |

Looking to reuse your own instructions? Create slash commands with [custom prompts](./prompts.md).

### Memory with AGENTS.md

You can give Cass extra instructions and guidance using `AGENTS.md` files. Cass looks for them in the following places, and merges them top-down:

1. `~/.cass/AGENTS.md` - personal global guidance
2. Every directory from the repository root down to your current working directory (inclusive). In each directory, Cass first looks for `AGENTS.override.md` and uses it if present; otherwise it falls back to `AGENTS.md`. Use the override form when you want to replace inherited instructions for that directory.

For more information on how to use AGENTS.md, see the [official AGENTS.md documentation](https://agents.md/).

### Tips & shortcuts

#### Use `@` for file search

Typing `@` triggers a fuzzy-filename search over the workspace root. Use up/down to select among the results and Tab or Enter to replace the `@` with the selected path. You can use Esc to cancel the search.

#### Esc–Esc to edit a previous message

When the chat composer is empty, press Esc to prime “backtrack” mode. Press Esc again to open a transcript preview highlighting the last user message; press Esc repeatedly to step to older user messages. Press Enter to confirm and Cass will fork the conversation from that point, trim the visible transcript accordingly, and pre‑fill the composer with the selected user message so you can edit and resubmit it.

In the transcript preview, the footer shows an `Esc edit prev` hint while editing is active.

#### `--cd`/`-C` flag

Sometimes it is not convenient to `cd` to the directory you want Cass to use as the "working root" before running Cass. Fortunately, `cass` supports a `--cd` option so you can specify whatever folder you want. You can confirm that Cass is honoring `--cd` by double-checking the **workdir** it reports in the TUI at the start of a new session.

#### `--add-dir` flag

Need to work across multiple projects in one run? Pass `--add-dir` one or more times to expose extra directories as writable roots for the current session while keeping the main working directory unchanged. For example:

```shell
cass --cd apps/frontend --add-dir ../backend --add-dir ../shared
```

Cass can then inspect and edit files in each listed directory without leaving the primary workspace.

#### Shell completions

Generate shell completion scripts via:

```shell
cass completion bash
cass completion zsh
cass completion fish
```

#### Image input

Paste images directly into the composer (Ctrl+V / Cmd+V) to attach them to your prompt. You can also attach files via the CLI using `-i/--image` (comma‑separated):

```bash
cass -i screenshot.png "Explain this error"
cass --image img1.png,img2.jpg "Summarize these diagrams"
```
