const { workspace, window } = require("vscode");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

let client;

function activate(context) {
  // Look for rosy-lsp binary in PATH or common locations
  const config = workspace.getConfiguration("rosy");
  const serverPath = config.get("lspPath", "rosy");

  const serverOptions = {
    command: serverPath,
    args: ["lsp"],
    transport: TransportKind.stdio,
  };

  const clientOptions = {
    documentSelector: [
      { scheme: "file", language: "rosy" },
    ],
  };

  client = new LanguageClient(
    "rosy-lsp",
    "ROSY Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

function deactivate() {
  if (client) {
    return client.stop();
  }
}

module.exports = { activate, deactivate };
