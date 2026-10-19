//! Starts `ecko lsp` for Ecko buffers. Everything else in this extension -
//! highlighting, outline, brackets, snippets - is declarative and needs no code.

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct Ecko;

impl zed::Extension for Ecko {
    fn new() -> Self {
        Ecko
    }

    // The server is the `ecko` binary itself, so there is nothing to download:
    // use the one on the worktree's PATH, or the one `lsp.ecko.binary.path`
    // names in the user's settings.
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);

        let path = binary
            .as_ref()
            .and_then(|b| b.path.clone())
            .or_else(|| worktree.which("ecko"))
            .ok_or_else(|| {
                "ecko is not on your PATH. Install it with \
                 `curl -fsSL https://ecko.sh/install | sh`, or set \
                 lsp.ecko.binary.path in your Zed settings."
                    .to_string()
            })?;

        let args = binary
            .and_then(|b| b.arguments)
            .unwrap_or_else(|| vec!["lsp".to_string()]);

        Ok(zed::Command {
            command: path,
            args,
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(Ecko);
