# Ecko for Zed

[Ecko](https://ecko.sh) language support for [Zed](https://zed.dev).

- **Completion, diagnostics, hover, go to definition, references, rename,
  formatting and code actions**, from `ecko lsp`: the same compiler that runs
  your code, so the editor never disagrees with a build.
- **Highlighting, the outline panel, bracket matching and auto-indent**, from
  the [Tree-sitter grammar](https://github.com/ecko-lang/tree-sitter-ecko).
- **SQL highlighting inside `sql { ... }` blocks**, when Zed's SQL extension is
  installed.
- **Snippets** for functions, `ai` calls, match, loops, types and more: type
  `fn`, `ai` or `match` and pick from the completion list.

## Requirements

`ecko` on your `PATH`:

```bash
curl -fsSL https://ecko.sh/install | sh
```

The extension runs `ecko lsp` from whichever `ecko` your shell finds, so the
editor always matches the version you build with. To use a different binary,
set it in Zed's settings:

```json
{
  "lsp": {
    "ecko": {
      "binary": { "path": "/path/to/ecko", "arguments": ["lsp"] }
    }
  }
}
```

If the language server does not start, **zed: open log** says why.

## Install it locally

1. In Zed, run **zed: install dev extension** and pick this `zed-ecko/`
   directory. Zed compiles `src/ecko.rs` to WebAssembly, which needs Rust
   installed through rustup; it adds the `wasm32-wasip2` target itself. It also
   clones `ecko-lang/tree-sitter-ecko` at the `rev` in `extension.toml` and
   builds the grammar.
2. Open a `.ecko` file. The language picker in the status bar should say Ecko.

After editing a query or the Rust, click **Rebuild** next to Ecko on the
Extensions page. To try a grammar change before pushing it, commit it to a
local clone and set `repository = "file:///absolute/path/to/clone"` and
`rev = "<sha>"`.

## Layout

| path | what it is |
|---|---|
| `extension.toml` | the manifest: the language server, the snippets, and the grammar's repository and revision |
| `src/ecko.rs`, `Cargo.toml` | the code that finds `ecko` and starts `ecko lsp` |
| `snippets/ecko.json` | a copy of `vscode-ecko/snippets/ecko.json` in the editors repo; CI fails if they differ |
| `languages/ecko/config.toml` | file suffix, comments, brackets, indentation |
| `languages/ecko/highlights.scm` | a copy of the grammar's `queries/highlights.scm`; CI fails if they differ |
| `languages/ecko/injections.scm` | SQL inside `sql { }` blocks |
| `languages/ecko/brackets.scm` | bracket and quote pairs |
| `languages/ecko/indents.scm` | indent inside `{ }`, `[ ]` and `( )` |
| `languages/ecko/outline.scm` | functions, types, templates and top-level bindings |

## License

MIT. See [LICENSE](LICENSE).
