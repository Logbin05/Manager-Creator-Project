# Manager Creator Project

An interactive terminal wizard that scaffolds a new project with the libraries you actually want — pick a template, answer a few questions, get a ready-to-run project.

Every stack is a **template**: a folder of data, not code. Adding support for a new language or framework means writing one TOML file and a few starter files — no Rust required. See [Adding a template](#adding-a-template).

```
┌  Manager Creator Project · new project
│
◇  What are we creating?
│  React · Frontend
│
◇  Project name
│  my-app
│
◇  Package manager
│  bun
│
◇  Architecture
│  Feature-Sliced Design
│
◇  Additional libraries
│  react-router, tanstack-query, zustand
│
◇  All done! ────────────────────────────────╮
│                                            │
│  Project  /home/me/projects/my-app         │
│  Run      cd my-app && bun run dev         │
│                                            │
├────────────────────────────────────────────╯
│
└  Happy hacking!
```

## Built-in templates

| Template | Base | Options |
|---|---|---|
| **Rust · Backend** | axum, tokio, serde, tracing | HTTP/1.1, HTTP/2 (h2c), HTTP/3 (stub); extra crates: sqlx, redis, tower-http, thiserror, anyhow, dotenvy, uuid |
| **React · Frontend** | Vite + React + TypeScript | npm / bun / pnpm; architecture: Modular, Feature-Sliced Design, Feature Oriented, Atomic Design, Components/Containers; extra libraries: react-router, tanstack-query, axios, zustand, redux-toolkit, react-hook-form (+ zod), tailwindcss and more |
| **Vanilla · Frontend** | HTML, CSS, JS | Plain files (no build step) or Vite; extra libraries: alpinejs, htmx, lit, sass, chart.js, swiper and more |

## Installation

### Prebuilt binaries (recommended)

No Rust toolchain needed. The installer downloads the right binary for your platform and adds it to your `PATH`.

**macOS / Linux**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Logbin05/manager-creator-project/releases/latest/download/manager-creator-project-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Logbin05/manager-creator-project/releases/latest/download/manager-creator-project-installer.ps1 | iex"
```

You can also download an archive for your platform from the [Releases](https://github.com/Logbin05/manager-creator-project/releases) page and put the `mcp` binary anywhere on your `PATH`.

### With cargo

```sh
cargo install --git https://github.com/Logbin05/manager-creator-project
```

### From source

```sh
git clone https://github.com/Logbin05/manager-creator-project
cd manager-creator-project
cargo install --path .
```

## Requirements

The CLI itself has no runtime dependencies. Each template needs its own tools, and the wizard checks for them **before** generating anything — you get a message with an install link instead of a half-created project.

| Template | Needs |
|---|---|
| Rust · Backend | `cargo` — [rustup](https://rustup.rs) |
| React, Vanilla + Vite | [Node.js](https://nodejs.org), [bun](https://bun.sh) or [pnpm](https://pnpm.io) |
| Vanilla, plain files | nothing |

## Usage

Run `mcp` in the directory where the new project folder should be created:

```sh
cd ~/projects
mcp
```

### Start screen

| Key | Action |
|---|---|
| `↑` `↓` / `k` `j` | Move selection |
| `Enter` | Open the selected item |
| `n` `t` `r` `s` | Jump straight to an item |
| `q` / `Esc` / `Ctrl+C` | Quit |

### Wizard

| Key | Action |
|---|---|
| `↑` `↓` | Move |
| `Space` | Toggle an item in a multi-select |
| `Enter` | Confirm the step |
| `Esc` | Go back one step (from the first step: back to the menu) |

Answers are remembered when you go back, so you can change one choice without redoing the rest. The last step shows a summary; answering **No** restarts the wizard with your answers prefilled.

Steps that don't apply are skipped: a template with a single package manager never asks which one to use, and one without architectures skips that question.

### Project names

A name must start with a letter and may contain Latin letters, digits, `-` and `_`. When the template uses an npm-compatible manager, npm rules also apply: no uppercase letters and no leading `_`. A folder with that name must not already exist.

### Settings

The `s` item on the start screen opens settings: update checks, default package manager, where projects are created, `git init` after creation, and an editor command to open the finished project. The path to the config file is shown at the bottom of that screen.

---

# Adding a template

A template is a folder under `templates/`. It holds a manifest (`template.toml`) and, optionally, starter files. Templates are embedded into the binary at compile time, so a merged template ships with the next release — users get it without downloading anything at runtime.

You don't need to write Rust. You need to know the commands your ecosystem uses to create a project and install a package.

## 1. Folder layout

```
templates/<id>/
├── template.toml          # the manifest — the only required file
├── files/                 # copied into every project from this template
│   └── src/main.js
└── layouts/               # optional: one folder per architecture/variant
    ├── plain/
    └── vite/
        └── vite.config.js
```

The folder name is the template `id` and must match the `id` field in the manifest.

## 2. The manifest

A minimal working example — a Go service:

```toml
schema = 1
id = "go-chi"
name = "Go · HTTP service"
hint = "chi router, standard layout"
author = "your-github-handle"
run_hint = "go run ./cmd/server"

[toolchain]
manager = ["go"]

[[steps]]
phase = "create"
program = "go"
args = ["mod", "init", "{{name}}"]
cwd = "project"

[[steps]]
phase = "setup"
program = "go"
args = ["get", "github.com/go-chi/chi/v5"]
```

### Top-level fields

All scalar fields go **before** the first `[section]` — that's how TOML works, and putting `run_hint` after `[toolchain]` is the most common mistake (the parser will say `unknown field run_hint, expected manager or requires`).

| Field | Required | Meaning |
|---|---|---|
| `schema` | yes | Manifest format version. Currently `1`. |
| `id` | yes | Must equal the folder name. |
| `name` | yes | Shown in the "What are we creating?" list. |
| `hint` | yes | One-line description next to the name. |
| `author` | yes | Your GitHub handle. |
| `order` | no | Sort order in the list. Defaults to `100`; built-in templates use `0`. |
| `run_hint` | no | Command shown at the end, without `cd`. Example: `"{{manager}} run dev"`. |
| `remove` | no | Paths to delete after the create step — for files a scaffolder leaves behind that your template replaces. |

### `[toolchain]`

```toml
[toolchain]
manager = ["npm", "bun", "pnpm"]   # interchangeable; more than one → the wizard asks
requires = ["git"]                 # always needed, never asked about
```

Both lists take **tool ids from the registry** (see [Tools](#3-tools)). If `manager` has one entry, the wizard picks it silently. If it's empty, the template can't use `{{manager}}` in its steps.

### `[[steps]]`

Each step runs one command. Steps execute in phase order, and within a phase in the order they appear.

| Field | Meaning |
|---|---|
| `program` | A tool id (`"cargo"`, `"go"`), or `"{{manager}}"` / `"{{manager.runner}}"` |
| `args` | Arguments as an array. Never one string with spaces. |
| `phase` | `"create"`, `"setup"` (default) or `"libs"` |
| `cwd` | `"parent"` or `"project"`. Defaults to `parent` for create, `project` for the rest. |
| `each` | Only in the libs phase: run the command once per selected library instead of once for all. |

**Phases** decide what happens when:

1. **create** — the project skeleton: `cargo new`, `npx create-vite`. Runs in the parent directory, because most scaffolders create the folder themselves. If no create step exists, the generator creates the folder for you.
2. *(files are copied here — see [Files and placeholders](#4-files-and-placeholders))*
3. **setup** — base dependencies and configuration. Runs inside the project.
4. **libs** — installs what the user picked. Skipped entirely if nothing was selected.

**`{{manager}}` vs `{{manager.runner}}`**: the first is the package manager itself (`npm`, `bun`), the second runs a package without installing it (`npx`, `bunx`, `pnpm dlx`). Required flags like `npx --yes` are added automatically — don't write them.

### `[[layouts]]` — the architecture choice

A layout is a variant of the same template: a project structure, a protocol version, a build setup. The wizard offers them as "Architecture".

```toml
[[layouts]]
id = "fsd"
label = "Feature-Sliced Design"
hint = "app → pages → widgets → …"
```

| Field | Meaning |
|---|---|
| `id` | Must match the folder name under `layouts/`, if it has one |
| `label`, `hint` | Shown in the list |
| `run_hint` | Overrides the manifest's `run_hint` for this variant |
| `steps` | Extra steps, merged with the manifest's steps in the same phases |
| `remove` | Paths to delete, applied together with the manifest's `remove` |
| `no_libs` | `true` — this variant can't install packages, so the wizard won't ask |

A layout must have **either** a folder under `layouts/` **or** its own steps. Files only, steps only, or both.

Here's a variant that changes dependencies rather than files — HTTP versions in the Rust template:

```toml
[[layouts]]
id = "h2"
label = "HTTP/2"
hint = "axum + h2c, no TLS"
[[layouts.steps]]
phase = "setup"
program = "cargo"
args = ["add", "axum", "--features", "http2"]
```

And one that changes the whole build setup — Vite in the vanilla template:

```toml
[[layouts]]
id = "vite"
label = "Vite"
hint = "dev server, hot reload"
run_hint = "{{manager}} run dev"
remove = ["main.js", "counter.js", "style.css"]   # create-vite's own starter files
[[layouts.steps]]
phase = "create"
program = "{{manager.runner}}"
args = ["create-vite@latest", "{{name}}", "--template", "vanilla", "--no-interactive"]
[[layouts.steps]]
phase = "setup"
program = "{{manager}}"
args = ["install"]
```

### `[[libs]]` — optional libraries

```toml
[[libs]]
label = "sqlx"
packages = ["sqlx", "--features", "runtime-tokio,postgres"]
hint = "PostgreSQL, migrations"
```

`packages` is passed as arguments to the libs step, so it can carry flags. One entry may install several packages:

```toml
[[libs]]
label = "react-hook-form"
packages = ["react-hook-form", "zod", "@hookform/resolvers"]
hint = "forms + validation"
```

If your libs need per-package flags (like cargo features), add `each = true` to the libs step so each entry gets its own command.

`[[libs]]` needs a libs step to work:

```toml
[[steps]]
phase = "libs"
program = "{{manager}}"
args = ["add", "{{packages}}"]
```

## 3. Tools

Templates can only run programs from the registry in `src/toolchain.rs`. Arbitrary shell commands, scripts and `postinstall` hooks are not accepted — that's what keeps a merged template from running anything on users' machines.

Currently available: `cargo`, `npm`, `pnpm`, `bun`, `deno`, `go`, `uv`, `poetry`, `composer`, `dotnet`, `git`.

If your ecosystem needs a tool that isn't there, open **two PRs**:

1. `tool: <id>` — one entry in `TOOLS`: binary name, Windows name if different, runner if any, install URL. Small and quick to review.
2. `template: <id>` — the template itself.

## 4. Files and placeholders

Files under `files/` are copied into every project from this template. Files under `layouts/<id>/` are copied after them and may overwrite them — that's how a variant replaces a starter file.

Copying happens **after** the create step, so your files land on top of whatever the scaffolder produced. Use `remove` for files the scaffolder leaves behind that yours don't replace.

Inside text files and step arguments you can use:

| Placeholder | Becomes |
|---|---|
| `{{name}}` | The project name the user typed |
| `{{manager}}` | The chosen package manager's binary (`npm`, `bun`) |
| `{{packages}}` | Only in a libs step: expands to all selected packages |

Binary files (images, fonts) are copied as-is.

## 5. Test it

```sh
cargo test                     # validates every manifest in templates/
cargo run                      # try your template end to end
```

`cargo test` is what catches an unknown tool, a broken layout reference, an unsafe `remove` path or a malformed manifest. It runs on every PR, so a template that fails it won't be merged.

Then check that the generated project actually starts — run the `run_hint` command and see it work.

If `cargo test` doesn't pick up your changes, the embedded snapshot is stale: `touch src/templates/template.rs` and run again (`build.rs` normally handles this).

## 6. Open a PR

Title it `template: <id>`. In the description say:

- what stack it's for and who'd use it,
- what you ran to verify it (which layout, which libraries),
- anything unusual — a non-obvious scaffolder flag, why a file is there.

Rules for review:

- `program` must come from the registry. No shell, no scripts.
- No network access beyond the scaffolding tool and package manager themselves.
- No pinned versions — let the package manager resolve current ones.
- No secrets, no vendored third-party code, no binaries over 100 KB.
- Keep starter files minimal: a working skeleton, not a demo app.

Templates ship with the next release after merge.

---

## Project structure (for contributors working on the CLI)

```
src/
├── main.rs                 # thin binary: calls the library and reports errors
├── lib.rs                  # main loop: start screen → wizard → generator
├── toolchain.rs            # the tool registry and the "is it installed" check
├── process.rs              # runs external commands
├── spec.rs                 # ProjectSpec: chosen template + answers
├── wizard.rs               # step machine, skips steps a template doesn't need
├── config.rs               # settings file
├── update.rs               # GitHub release check
├── templates/
│   ├── structures.rs       # manifest types
│   ├── template.rs         # loading, validation, the embedded templates/ dir
│   └── generate.rs         # runs the steps, copies the files
├── settings/               # settings screen: mod.rs · render.rs · actions.rs
└── ui/
    ├── guard.rs            # raw-mode guard: restores the terminal even on panic
    ├── logo.rs             # responsive logo (4 sizes) with truecolor gradient
    └── start_screen.rs     # main menu
```

The CLI knows nothing about specific stacks. It loads manifests, asks the questions they declare, and runs the steps they list. Everything stack-specific lives in `templates/`.

### Development

```sh
cargo test                # manifests, menu logic, logo selection
cargo run                 # run from the repo
cargo install --path .    # reinstall the `mcp` command after changes
```

Tip: run the installed `mcp` from a scratch directory (`/tmp`, for example) so generated projects don't land inside the repo.

## Roadmap

- TLS (rustls + rcgen dev certificates) for the HTTP/2 template
- A real HTTP/3 template
- More templates: Go, Python, Vue, Svelte
- "From template" and "Recent projects" menu items

## License

MIT