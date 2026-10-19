# Ecko for Zed

[Ecko](https://ecko.sh) language support for [Zed](https://zed.dev): syntax
highlighting, the outline panel, bracket matching and auto-indent for `.ecko`
files, from the Tree-sitter grammar in `../tree-sitter-ecko`.

Diagnostics, completion and the rest of what `ecko lsp` serves are not wired up
yet. Zed starts a language server through extension code compiled to
WebAssembly, which this extension does not have.

## Install it locally

Zed builds the grammar from a git repository at a pinned revision, so the
grammar has to be committed somewhere it can clone. Until
`ecko-lang/tree-sitter-ecko` is published, use a local clone:

1. Commit `../tree-sitter-ecko` (with its generated `src/`) to a local git
   repository and note the commit SHA.
2. In `extension.toml`, set `[grammars.ecko]` to
   `repository = "file:///absolute/path/to/that/repo"` and `rev = "<sha>"`.
3. In Zed, run **zed: install dev extension** and pick this `zed-ecko/`
   directory.
4. Open a `.ecko` file. **zed: open log** shows any grammar build error.

## Layout

| path | what it is |
|---|---|
| `extension.toml` | the manifest, and the grammar's repository and revision |
| `languages/ecko/config.toml` | file suffix, comments, brackets, indentation |
| `languages/ecko/highlights.scm` | a copy of `../tree-sitter-ecko/queries/highlights.scm`; CI fails if they differ |
| `languages/ecko/brackets.scm` | bracket and quote pairs |
| `languages/ecko/indents.scm` | indent inside `{ }`, `[ ]` and `( )` |
| `languages/ecko/outline.scm` | functions, types, templates and top-level bindings |

## Publishing

Zed's registry is a pull request to
[`zed-industries/extensions`](https://github.com/zed-industries/extensions)
adding the extension as a submodule. Its rules that matter here:

- The extension's repository must be public and carry an accepted open-source
  licence. This one and the grammar are both MIT, and each is published as its
  own public repository.
- A human must open the pull request and answer review questions; the
  registry does not accept contributions from autonomous agents.

Once the repositories exist: pin the grammar's real commit in
`extension.toml`, test the dev install, then follow Zed's
[publishing guide](https://zed.dev/docs/extensions/publishing/overview).

## License

MIT. See [LICENSE](LICENSE).
