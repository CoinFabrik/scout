import * as vscode from "vscode";
import * as commandExists from "command-exists";

const RUST_ANALYZER_CONFIG = "rust-analyzer.check.overrideCommand";

export async function activate(_context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration();

  // Check rust-analyzer is installed
  if (!config.has(RUST_ANALYZER_CONFIG)) {
    await vscode.window.showErrorMessage(
      "rust-analyzer must be installed in order for scout to work"
    );
  }

  // Check scout is installed
  try {
    await commandExists("cargo-scout");
  } catch (err) {
    await vscode.window.showErrorMessage(
      "cargo-scout must be installed in order for scout to work"
    );
    return;
  }

  // Update settings to change rust-analyzer config
  await config.update(RUST_ANALYZER_CONFIG, [
    "cargo",
    "scout",
    "--",
    "--message-format=json",
  ]);
}

export function deactivate() {
  // unused
}
