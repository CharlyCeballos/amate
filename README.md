# Amate

English · [Español](README.es.md)

**A 100% Rust text editor: it starts instantly, can be used entirely with a keyboard or a screen
reader, and connects to nothing.**

Amate is an open-source desktop editor for text and code, for Windows, Linux (with priority for
Debian and its derivatives) and macOS. It is meant for people who use Notepad++ on Windows and want
the same on Linux and macOS ([Q17]). The name comes from the Nahuatl *āmatl*, the bark paper of the
codices ([Q01]). The planning documents are in Spanish ([Q08]), so this README keeps their terms:
milestones are *hitos* (Hito 0 to Hito 5, plus Hito 4b), and the parity levels are `familiar`,
`diario` (daily) and `completo` (complete).

## Status

Hito 0, in progress: a Cargo workspace with CI and an empty window built with egui; there is
nothing to download yet. Next to the code live the documents that prepare it: the operative
prompt, the decisions, the analysis of Notepad++, the questionnaire, the roadmap and the
distribution plan. The `notepadrust`
folder name is only a local working name ([Q01]).

The context is fixed: one person with solid Rust experience works with a coding agent. The
decisions in [DECISIONES.md](docs/DECISIONES.md) were taken between 2026-10-15 and 2026-10-20, and
all of them have an answer except the visual likeness to Notepad++ ([Q12]), which applies its
recommendation as an assumption.

What a public project carries arrives in stages ([Q15]): [LICENSE](LICENSE) and this README, in
English and Spanish, from the first commit; CI and `SECURITY.md`, with GitHub's private
vulnerability reporting, in Hito 0, together with the first code; `CONTRIBUTING.md` with the DCO
([Q22]), the code of conduct and the issue templates, in the first public release; and
`GOVERNANCE.md`, once there are three or more recurring contributors.

## What Amate is and is not

Amate is:

- **The spiritual successor of Notepad++** ([Q18]): the same philosophy (fast, light, opens any
  text file without ceremony), a recognizable workflow and shortcuts, and the freedom to fix what
  is badly designed. On Windows and Linux it uses the same shortcuts as Notepad++; on macOS, Cmd
  where Notepad++ uses Ctrl, respecting the shortcuts the system reserves ([Q23]).
- **Strict Rust** ([Q19]): no C or C++ compiled into the distributed binary; it does use the
  operating system's APIs and libraries. The interface is native: it runs in-process, is written in
  Rust, and uses neither Electron nor a webview as its main interface ([Q16]).
- **Fast, with measured budgets** ([Q25]): from Hito 1, CI measures cold start (under 300 ms),
  keystroke latency (under 16 ms at p99) and idle RAM with a 1 MB file (under 100 MB), and fails on
  a regression.
- **Accessible by requirement** ([Q20]): it can be used entirely with a keyboard from Hito 1, and
  the screen reader (NVDA, VoiceOver and Orca) is verified before the first public release. Hito 0
  measures it together with the IME, and a toolkit that fails is discarded ([Q24]).
- **No network** ([Q13]): no version check and no telemetry. New versions are downloaded from
  GitHub Releases.
- **A product for end users**: it installs, starts, opens files and updates without anyone having
  to know what Rust is.

Amate is not:

- **A fork or a port of Notepad++** ([Q18]). It is a clean reimplementation: it does not copy or
  translate code, comments, manual text, icons or resources, and no Notepad++ file ships in its
  packages ([Q21]).
- **An IDE.** A feature that makes Amate slower to start, or no longer understandable at a glance,
  is out of scope, unless the maintainer decides otherwise.
- **A promise to replicate every Notepad++ command.** What goes beyond `diario` is decided by real
  user requests, never by the menu list ([Q03]).
- **Compatible with Notepad++ plugins.** DLL plugins are never a target, and there is no plugin
  system before 1.0 ([Q06]).
- **An importer of Notepad++ configuration.** Amate has its own configuration ([Q05]).
- **An editor for multi-GB files, or a hex editor.** A 10 MB file opens, scrolls and edits
  smoothly; above that, a degraded mode starts in which the interface never freezes, but with no
  promise of smoothness ([Q11], [Q17]).

## What the analysis found

The analysis in [01-analisis-notepad-plus-plus.md](docs/01-analisis-notepad-plus-plus.md) measured
the Notepad++ repository on a shallow clone of `main` on 2026-10-15. These are the findings that
changed the project the most, each with the decision that was taken.

1. **It is not a port: it is a reimplementation.** Notepad++ has no Linux or macOS support:
   `SUPPORTED_SYSTEM.md` lists only Windows 8.1, 10 and 11. Its interface is 30 subdirectories of
   Win32 controls in `PowerEditor/src/WinControls/`, defined in `.rc` resource files; its dark mode
   is built with Win32 *subclassing* (`NppDarkMode.cpp`) and its DPI handling is Windows-specific
   (`dpiManagerV2`) (verified: 2026-10-15). There is nothing to port.

   **Decision:** Amate is a spiritual successor, neither fork nor port ([Q18]), and a clean
   reimplementation with nothing of Notepad++ inside it ([Q21]). Its interface is native and
   written in Rust ([Q16]). On Windows it promises 10 (22H2) and 11 ([Q09]); Windows 8.1 is not on
   the list.

2. **Notepad++ does not implement text editing: it drives Scintilla.** Of the roughly 311 000 lines
   of code in the repository (only `.cpp`, `.h` and `.cxx`, without Boost.Regex, resources or
   XML), 158 145 are the application (`PowerEditor/src`), 73 252 are Scintilla, the editing engine,
   which has existed since 1999, and 79 583 are Lexilla, with 128 highlighting lexers
   (verified: 2026-10-15). Rewriting that in Rust or linking it through FFI was the project's
   number-one decision: it decided whether to rewrite 158 000 lines (only the application) or
   311 000 (the engine and the lexers too). Scintilla and Lexilla carry their own permissive
   license, HPND style (verified: 2026-10-15), so linking them was legally viable.

   **Decision:** [Q19] settled it the long way: no C or C++ compiled into the binary, so
   Scintilla, Lexilla and Tree-sitter are out. Amate writes its own engine in Rust, with the
   document in a rope from Hito 1 ([Q11]), and highlights with `syntect` and `.sublime-syntax`
   grammars, which are data, not code ([Q19], [Q26]). The size reference is the total, not just the
   application.

3. **The plugin ecosystem cannot be inherited.** Notepad++ plugins are Windows DLLs that talk to
   Notepad++ through Win32 `SendMessage`, with 326 messages (`NPPM_*` and `NPPN_*` in
   `Notepad_plus_msgs.h`), and manipulate Scintilla's window handles directly. The official
   catalog, `nppPluginList`, has 1 701 stars on GitHub, which measure its popularity, not how many
   plugins it has (verified: 2026-10-15). It is an ecosystem to replace, not to migrate.

   **Decision:** [Q06] is `despues` (later): there is no plugin system before 1.0, and Notepad++
   DLL plugins are never a compatibility target. From Hito 1, every user action is a named,
   serializable command, which enables macros, a command palette, configurable shortcuts and
   automated tests. Lua is out because `mlua` compiles C ([Q19]); the future candidates are Rhai,
   or WebAssembly with `wasmi`, both pure Rust.

4. **Notepad++ configuration is declarative, but Amate does not import it.** Its data is
   declarative XML: 95 languages with built-in highlighting (`langs.model.xml`), 22 themes, 94
   interface translations and 48 *Function List* definitions, plus the community collection of
   user-defined languages (UDL), `userDefinedLanguages`, with 823 stars on GitHub, which measure
   popularity, not the number of languages (verified: 2026-10-15). That is why importing that
   configuration looked like the cheapest adoption lever.

   **Decision:** [Q05] is `ninguna` (none): Amate has its own configuration (recommended format:
   TOML) and there are no importers on the roadmap. Hito 3 puts in writing the estimated cost of
   importing themes, UDL, shortcuts and sessions, as excluded work. No Notepad++ file (themes, UDL,
   translations) ships in the packages, even though the GPL would allow it ([Q21]). That is why
   Amate does not present itself as “compatible with Notepad++ configuration”.

5. **“Parity with Notepad++” is not a verifiable goal.** The main menu (`IDR_M30_MENU` in
   `Notepad_plus.rc`) has 510 items spread over 13 top-level menus. `Edit → Line Operations` alone
   has 26 commands, 14 of them line-sorting variants, and `Edit → Convert Case to` has 8 variants
   (verified: 2026-10-15). Explicit parity levels have to be defined, accepting that the long tail
   arrives late or never: chasing all of it is the standard way clones of mature software die.

   **Decision:** [Q03] sets three named levels. `familiar` is the Hito 1 baseline, polished.
   `diario` (daily) adds sessions with recovery of unsaved tabs, find in files, column editing and
   multiple cursors, line operations (duplicate, move, delete, comment, upper and lower case,
   indentation), folding, split view with two views of the same buffer, a working folder, the
   languages of [Q26], and configurable themes and shortcuts; no plugins. `completo` (complete)
   adds macros, user-defined languages, document map, function list, printing and autocompletion.
   The first public release has `diario` parity, delivered in Hito 2. `completo` and the long tail
   are decided by real user requests, never by the Notepad++ menu list.

**And a correction about the distribution reference.** Amate's distribution reference is the
*standalone* section of the PhotoCraft page, a native Rust application for the three systems. For
Linux, the 0.7.0 page offers only Flatpak; release 0.9.0 publishes more Linux files
(to be verified). Whatever its real list is, Amate's Linux matrix comes from the requirement of
Debian and its derivatives, not from PhotoCraft: Flatpak is not preinstalled on Debian or Ubuntu
(to be verified), and its sandbox routes disk access through XDG portals (to be verified), a
constant friction for a text editor.

**Decision:** [Q09] makes the amd64 `.deb` a first-class artifact for Debian 12+ and
Ubuntu 24.04+ (Linux Mint 22+ by inheritance), with AppImage and `.tar.gz` for the other
distributions; Flatpak waits for Hito 4b. Linux is built in a `debian:bookworm` container and
tested in clean containers. From the PhotoCraft model, Amate takes what does fit: the application
installs on its own, with no launcher; on Windows there is an MSI installer and a portable ZIP
(“unzip and run”); binaries live on GitHub Releases, the single channel, with `SHA256SUMS.txt` and
the provenance attestation ([Q28]); and a new version is obtained by downloading the new release
([Q13]).

## Decisions

### What is settled

This table summarizes the decisions that weigh most on the first impression of people who come
from outside ([Q31]). All of them, from Q01 to Q31, with what each one implies and which part of
the plan moves if it changes, are in [DECISIONES.md](docs/DECISIONES.md), the source of truth. The
documents cite them by their identifier, like this: [Q09].

| Topic | Decision | ID |
| --- | --- | --- |
| License | `GPL-3.0-or-later` for the whole repository: code, planning documents and the future manual. | [Q02] |
| Parity of the first public release | `diario` (daily), delivered in Hito 2. | [Q03] |
| Interface | `nativa` (native): it runs in-process and is written in Rust. No Electron and no webview as the main interface. | [Q16] |
| Native code in the binary | Strict Rust: no C or C++ compiled into the distributed binary; the operating system's APIs and libraries are used. | [Q19] |
| Accessibility | A requirement that filters the toolkit: full keyboard use from Hito 1, and the screen reader verified before the first public release. | [Q20] |
| Languages | Interface in Spanish and English in the first public release. Bilingual README. Manual in the same languages as the interface, in Hito 3. Planning in Spanish; issues in Spanish or English; commits, code and comments in English. | [Q07], [Q08] |
| File size | `normal`: a 10 MB file opens, scrolls and edits smoothly; above that, degraded mode. | [Q11] |
| Highlighted languages | 13 in Hito 1 and 32 in the first public release. | [Q26] |
| Notepad++ configuration | `ninguna` (none): Amate has its own configuration and there are no importers. | [Q05] |
| Own plugins | `despues` (later): none before 1.0. | [Q06] |
| Packages | Windows 10 (22H2) and 11, x64: MSI and portable ZIP. macOS, universal: DMG. Debian 12+ and Ubuntu 24.04+, amd64: `.deb`. Rest of Linux, x86_64: AppImage and `.tar.gz`. Single channel: GitHub Releases. Out unless explicitly requested: Windows x86, `.rpm`, web/WASM, FreeBSD, RISC-V, Snap and the Mac App Store. | [Q09] |
| Signing | `despues` (later) for paid certificates. In Hito 4 the project applies for SignPath Foundation's free code signing for open source, whose requirements still have to be checked (to be verified); if approved, the first public release is already signed on Windows. Without a signature, anyone with Windows 11's Smart App Control turned on cannot open Amate (to be verified). | [Q10] |
| Updates | `release`: new versions are downloaded from GitHub Releases. Amate makes no network connection at all: no version check and no telemetry. | [Q13] |
| Versioning and provenance | SemVer, with 0.x until signing. Every release publishes `SHA256SUMS.txt` and the GitHub provenance attestation. | [Q27], [Q28] |
| Success criterion | External adoption: people from outside download it, open issues and contribute. Without telemetry, it is measured with public GitHub data. | [Q31] |

### What is still open

- **The visual likeness to Notepad++** ([Q12]) has no answer, so its recommendation applies as an
  assumption: `familiar`, with menus, tabs on top, a status bar and Notepad++ shortcuts for the
  actions that exist, and its own icons and brand.
- **The trademark and the domain.** Before publishing any artifact, creating the public repository
  or reserving the crate, the “Amate” trademark (IMPI, USPTO, EUIPO) and the domain still have to
  be verified ([Q01]).
- **The technical choices the decisions do not fix:** the interface toolkit, the specific crates
  (the rope one, encodings, configuration) and the packaging tools. The executable plan compares
  the alternatives for each one and recommends one; Hito 0 confirms the toolkit with measurements
  ([Q24]). Every new crate or tool is proposed with at least three candidates and a security
  comparison (maintenance, known advisories, adoption, transitive dependencies and license), and
  it enters only once the maintainer approves it.
- **The questions that no decision closed** remain open in
  [02-cuestionario.md](docs/02-cuestionario.md), by urgency and each with its recommendation.
- **The gaps the plan has to close**, each with a suggestion, are in the section
  “Huecos que el plan debe cerrar” (gaps the plan must close) of [PROMPT.md](docs/PROMPT.md).
- **Dated facts** (versions, support dates, third-party requirements) are checked again on every
  run of the prompt; the list is in the section “Hechos por re-verificar” (facts to re-verify) of
  [fusion.md](docs/fusion.md).

## How to continue

1. **Review [DECISIONES.md](docs/DECISIONES.md).** A `Respuesta:` line with content closes the
   decision; an empty one applies the recommendation, and that consequence is marked as an
   assumption in the plan. To change a decision, edit its answer: the “Si cambias” (“if you
   change”) part of each one says which part of the plan moves.
2. **Run the prompt.** Open a coding agent inside this repository and give it
   [PROMPT.md](docs/PROMPT.md) as the first message. The agent first reads DECISIONES.md and
   documents 01 to 04, verifies and extends what is already done instead of redoing it, and checks
   the dated facts again on the day it runs. It delivers a single plan with fixed headings
   (analysis, architecture, milestones, first batch, tests and CI, packaging, risks, questionnaire,
   applied assumptions and sources consulted) and saves it as `docs/05-plan-ejecutable.md`. That
   run writes no application code, opens no pull requests and creates no other repository: this is
   the product repository. ADRs are written in Hito 0, not in that run.
3. **Review the plan.** Its questionnaire only brings the decisions that are still unanswered or
   that came up during the run; new ones use the identifiers `N01`, `N02`…, so they never collide
   with those of the decisions (Q01 to Q31). If you change an answer, run the prompt again.
4. **Start Hito 0**, the one with the first code: the Cargo workspace, CI on the three systems and
   the measured validation of the toolkit.

### Roadmap

| Hito | What it delivers |
| --- | --- |
| Hito 0 — Foundations and measured validation | A Cargo workspace that compiles on Windows, Linux and macOS in CI: `fmt`, `clippy -D warnings`, tests, `cargo-deny` and the rule of [Q19] (CI fails if the binary's dependency tree includes `cc`, `cmake` or `bindgen`, except the `wayland-backend` case that [Q19] records). An empty window packaged roughly on each system, not yet the final MSI, DMG or `.deb`. Toolkit spikes on `spike/` branches, which get deleted, to measure IME, screen reader, idle consumption, keystroke latency, packaging cost and the macOS universal build; what remains is an ADR in `docs/adr/` with the GO/PIVOT decision ([Q20], [Q24]). With the first code come CI and `SECURITY.md` ([Q15]), and the threshold for opening a 10 MB file is set with measurements ([Q25]). Exit: green CI on the three systems, ADRs with numbers and a rough artifact downloadable from CI. |
| Hito 1 — First usable version (0.1.0, no announcement, [Q27]) | The “Base del hito 1” (Hito 1 baseline) of PROMPT.md plus the [Q04] additions: bracket matching, visible whitespace, the system's light or dark theme, HiDPI, configuration paths resolved at runtime with portable mode (a marker file next to the executable), and opening and saving ANSI (Windows-1252 and ISO-8859-1). Also: full keyboard use ([Q20]), strings outside the code ([Q07]), the 13 languages of [Q26], the document in a rope ([Q11]), the serializable command layer ([Q06]), a document model that already allows several views of the same buffer (split view arrives in Hito 2), the macOS shortcut mapping ([Q23]) and the [Q09] packages built in CI, unsigned ([Q10]). The [Q25] budgets are measured in CI from here on. The exit includes daily use: the maintainer uses Amate as the main editor for basic tasks for two weeks. |
| Hito 2 — Daily workflow | Everything in `diario` that did not go into Hito 1, with the 32 languages of [Q26], and find in files, folding and bidirectional text ([Q03], [Q04]). Exit: a regular Notepad++ user works a full day without going back, checked with real users. |
| Hito 3 — Manual and preferences | Editable preferences and themes, a user manual in Spanish and English ([Q08]), the complete interface in both languages ([Q07]) and, in writing, the estimated cost of importing Notepad++ configuration, as excluded work ([Q05]). |
| Hito 4 — Distribution and first public release (0.N) | The final [Q09] artifacts, `SHA256SUMS.txt` and the provenance attestation ([Q28]), desktop integration ([Q14]), the README download section with the steps to open unsigned builds, the application to SignPath Foundation ([Q10]) and `CONTRIBUTING.md` with the DCO, a code of conduct and issue templates ([Q15], [Q22]). Exit: one `git tag` produces everything with no manual step, and the packages are installed and verified on clean machines. |
| Hito 4b — optional, after Hito 4 | Flatpak, arm64 `.deb`, Windows ARM64, its own apt repository, winget, Homebrew cask, paid signing and macOS notarization ([Q09], [Q10]). It is kept apart from closing Hito 4. If downloads call for it, winget, Homebrew and apt are the first candidates to move earlier ([Q31]). |
| 1.0 | Not a milestone but a gate: `diario` complete, signing on Windows, notarization on macOS and the manual ([Q27]). |
| Hito 5 — Extensions | With [Q06] at `despues` (later), it is only a one-page annex of the plan, never scheduled work. |

Every milestone ends in something that can be run or installed, and it closes when its exit
criterion is met, not on a date: there are no calendar estimates. The detail of each one (work,
crates it touches, exit criterion, risks and mitigation) is in
[03-plan-de-trabajo.md](docs/03-plan-de-trabajo.md).

## Documents

- **To continue the work:** review DECISIONES.md and run PROMPT.md with a coding agent, as
  described in “How to continue”.
- **To understand the project:** read DECISIONES.md, then the analysis (01) and the roadmap (03).
  Distribution is in 04, and the reasoning behind each question, in 02.

| File | What it is |
| --- | --- |
| [docs/PROMPT.md](docs/PROMPT.md) | **The operative prompt.** A coding agent runs it inside this repository. It sets the rules that are not debated, applies the decisions, defines the Hito 1 baseline, lists the gaps the plan must close and requires a delivery format with fixed headings. Its output is `docs/05-plan-ejecutable.md`. |
| [docs/DECISIONES.md](docs/DECISIONES.md) | **The answer sheet and the source of truth.** From Q01 to Q31, each decision with its answer, what it implies and which part of the plan moves if it changes. |
| [docs/01-analisis-notepad-plus-plus.md](docs/01-analisis-notepad-plus-plus.md) | Analysis with measured data on a clone of the Notepad++ repository: the organization and its nine public repositories (verified: 2026-10-15), lines of code per component, functional surface, configuration formats, plugin API, licenses and trademark, what PhotoCraft teaches about distribution (and where its model does not fit) and what is still to be analyzed. |
| [docs/02-cuestionario.md](docs/02-cuestionario.md) | **The long questionnaire:** 56 questions in 10 blocks, each with its urgency (blocking, important or deferrable), why it matters, options and recommendation, and with its status: which decision closed it or, if it is still open, what is recommended. DECISIONES.md has the answers; this file, the reasoning. |
| [docs/03-plan-de-trabajo.md](docs/03-plan-de-trabajo.md) | The roadmap by milestones, without calendar estimates: relative size, goal, work, crates touched, exit criterion, risks and mitigation for each milestone, the 1.0 gate and the project's risk table ordered by probability. |
| [docs/04-distribucion-y-empaquetado.md](docs/04-distribucion-y-empaquetado.md) | Distribution: what to copy from PhotoCraft and what not, the artifact matrix by system and architecture, the glibc problem on Debian, portable mode, signing and notarization, desktop integration, the tools and the release pipeline from a tag, updates and the download section. |
| [docs/fusion.md](docs/fusion.md) | How the earlier planning drafts were merged into this repository: the source → destination map, the editorial decisions, the conflicts the owner decided, the discarded content and why, why the prompt is the way it is, and the facts to re-verify. |
| [LICENSE](LICENSE) | The text of the GPL-3.0 license. |
| [.gitattributes](.gitattributes) | LF across the repository, with CRLF only for `*.bat` and `*.cmd` ([Q30]). |

`docs/05-plan-ejecutable.md`, which the prompt run creates, does not exist yet. The ADRs live in
`docs/adr/`, starting with the interface toolkit ([0001](docs/adr/0001-toolkit.md)).

## License and trademark

Copyright © 2026 Carlos Ceballos and contributors.

The copyright holder is Carlos Ceballos, as an individual ([Q29]). Amate is distributed under the
`GPL-3.0-or-later` license ([Q02]), which covers the whole repository: the code, the planning
documents and the future manual. The full text is in [LICENSE](LICENSE). Dependencies must have a
license compatible with GPLv3; from Hito 0, `cargo-deny` checks this in CI.

Contributions use the DCO (`Signed-off-by`), with no CLA ([Q22]). Every commit written with the
help of an agent is reviewed and signed off by the maintainer, and `CONTRIBUTING.md` will ask the
same of anyone who contributes assisted code. Changing the license would require the permission of
every contributor ([Q02]). Issues are written in Spanish or English, and commits in English
([Q08]).

Notepad++ is a trademark of its owner. Amate is not affiliated with Notepad++ or its authors.
