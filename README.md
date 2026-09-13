# Manager Creator Project

An interactive terminal wizard that scaffolds a new project with the libraries you actually want — pick a stack, answer a few questions, get a ready-to-run project.

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

## Supported stacks

| Stack | Base | Options |
|---|---|---|
| **Rust · Backend** | axum, tokio, serde, tracing | HTTP/1.1, HTTP/2 (h2c), HTTP/3 (stub); extra crates: sqlx, redis, tower-http, validator, thiserror, anyhow, dotenvy, uuid |
| **React · Frontend** | Vite + React + TypeScript | Package manager: npm or bun; architecture: Modular, Feature-Sliced Design, Feature Oriented, Atomic Design, Components/Containers; extra libraries: react-router, tanstack-query, axios, zustand, redux-toolkit, react-hook-form (+ zod), clsx |

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

If you already have Rust installed:

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

The CLI itself has no runtime dependencies. The tools below are needed only to generate a project of the corresponding stack:

| Stack | Needs |
|---|---|
| Rust · Backend | `cargo` (via [rustup](https://rustup.rs)) |
| React · Frontend with npm | [Node.js](https://nodejs.org) (ships with `npm` and `npx`) |
| React · Frontend with bun | [bun](https://bun.sh) |

The wizard runs each tool in the background. If one is missing, you get a clear message instead of a half-created project.

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

### Project names

A name must start with a letter and may contain Latin letters, digits, `-` and `_`. For React projects, npm rules also apply: no uppercase letters and no leading `_`. A folder with that name must not already exist.

## What gets generated

### Rust · Backend

```
my-api/
├── Cargo.toml        # dependencies added with `cargo add`, so versions are always current
└── src/
    └── main.rs       # minimal axum server on 0.0.0.0:3000 with tracing
```

- **HTTP/1.1** — axum + tokio.
- **HTTP/2** — the same, with axum's `http2` feature. Runs h2c (HTTP/2 without TLS); test with `curl --http2-prior-knowledge localhost:3000`. Browsers require TLS for HTTP/2 — adding it is on the roadmap.
- **HTTP/3** — pulls quinn/h3 but generates a placeholder `main.rs` for now.

### React · Frontend

The project is created with the official Vite `react-ts` template, then the folders of the chosen architecture are added on top. Every layer gets an `index.ts` — its public API — with a one-line comment describing what belongs there.

**Modular** (modules with `domain / store / ui`, inspired by DDD)

```
src/
├── core/             # providers, router, global config
├── shared/ui/        # UI kit shared between modules
├── libs/             # wrappers over third-party libraries
└── modules/
    └── example/      # sample module: domain/, store/, ui/, index.ts
```

**Feature-Sliced Design**

```
src/
├── app/ ├── pages/ ├── widgets/ ├── features/ ├── entities/
└── shared/ ui/ api/ lib/
```

**Feature Oriented** — `features/`, `shared/components/`, `shared/hooks/`, `api/`

**Atomic Design** — `components/{atoms,molecules,organisms,templates}/`, `pages/`

**Components / Containers** — `api/`, `components/`, `containers/`, `store/`, `pages/`

Selected libraries are installed with `npm add` or `bun add`, so `package.json` and the lock file are always consistent.

## Project structure (for contributors)

Each stack is a self-contained module: its data model, its wizard questions, and its generator live together. Shared code only knows the list of stacks.

```
src/
├── main.rs                 # thin binary: calls the library and reports errors
├── lib.rs                  # main loop: start screen → wizard → generator
├── process.rs              # runs external commands (cargo, npm, bunx …)
├── spec.rs                 # ProjectSpec: common answers + per-stack answers
├── wizard.rs               # step machine: stack → name → stack questions → confirm
├── stacks/
│   ├── mod.rs              # enum Stack, dispatch to a generator
│   ├── rust/               # model.rs · questions.rs · generate.rs
│   └── react/              # model.rs · manager_packet.rs · questions.rs · generate.rs
├── ui/
│   ├── guard.rs            # raw-mode guard: restores the terminal even on panic
│   ├── logo.rs             # responsive logo (4 sizes) with truecolor gradient
│   └── start_screen.rs     # main menu
└── templates/              # files embedded into the binary at compile time
```

### Adding a new stack

1. Create `src/stacks/<name>/` with `model.rs`, `questions.rs`, `generate.rs` and a `mod.rs` that re-exports the public API.
2. Add a variant to `enum Stack` and a match arm in `stacks::generate`.
3. Add a `<name>: <Name>Answers` field to `ProjectSpec` (with `summary` and `run_hint`).
4. Add the new steps to `Step` in `wizard.rs` and an item to the "What are we creating?" prompt.

Existing stacks don't need to be touched.

### Development

```sh
cargo test                # menu logic and logo selection are unit-tested
cargo run                 # run from the repo
cargo install --path .    # reinstall the `mcp` command after changes
```

Tip: run the installed `mcp` from a scratch directory (`/tmp`, for example) so generated projects don't land inside the repo.

## Roadmap

- TLS (rustls + rcgen dev certificates) for the HTTP/2 template
- A real HTTP/3 template
- pnpm support
- "From template" and "Recent projects" menu items
- Templates as data (TOML) so new stacks can be added without recompiling

## License

MIT