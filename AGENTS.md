# AGENTS.md

## Core rule: read-only unless asked

Do not edit, create, delete, move, or rename files in this repository unless the
user has explicitly asked for that specific change. Do not "helpfully" fix
things you notice along the way.

Reading and exploring are always fine. Changing code is not.

## Do without asking

- Read files, search the codebase, list directories.
- Explain errors, trace causes, and quote the relevant code.
- Propose changes in chat: show the exact diff or code the user could paste.
- Run read-only inspection commands (e.g. `cargo check`, `cargo build`,
  `git status`, `git diff`, `git log`). These produce build artifacts but must
  not modify source files.

## Do NOT do without asking

- Edit any source, config, manifest, or doc file.
- Fix compile errors, warnings, typos, or formatting you encounter.
- Add, remove, or upgrade dependencies in `Cargo.toml` / `Cargo.lock`.
- Run commands that write to source or history: `cargo fmt`, `cargo fix`,
  `cargo add`, `git add`, `git commit`, `git checkout`, `git restore`,
  `git reset`, `git stash`, `git rebase`, formatters, codegen, migrations.
- Create scaffolding, tests, examples, or documentation files.
- Refactor, rename, reorganize, or delete "unused" or "dead" code.
- Modify uncommitted or in-progress work.

## When a change seems necessary

1. Describe the problem and the fix.
2. Show the proposed diff.
3. Wait for explicit approval before touching any file.

If the user asks you to fix or implement something, stay strictly within that
requested scope. Do not make unrelated or drive-by improvements in the same
change. If you spot an unrelated issue, report it separately instead of fixing it.

## Requests that count as approval

Only a clear, specific instruction counts, such as "fix the bind error in
`src/crud.rs`" or "apply that diff." Do not infer approval from questions like
"why is this failing?", "can this work?", or "what would you change?"
