import * as vscode from "vscode";
import {
    LanguageClient,
    type LanguageClientOptions,
    type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | null = null;

export async function activate(_cx: vscode.ExtensionContext) {
    if (client !== null) {
        return;
    }
    const config = vscode.workspace.getConfiguration("grustonnet");
    if (!config.get("server.enable")) {
        return;
    }
    const configPath = config.get("server.path");
    const serverOpts: ServerOptions = {
        command:
            typeof configPath === "string" && configPath.length !== 0
                ? configPath
                : "grustonnet-ls",
    };
    const clientOpts: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "jsonnet" }],
        initializationOptions: {
            root_dirs: ".",
        },
        synchronize: {
            // TODO: this is broken
            configurationSection: "config",
        },
    };
    client = new LanguageClient("grustonnet", serverOpts, clientOpts);
    await client.start();
}

export async function deactivate() {
    if (client === null) {
        return;
    }
    await client.stop();
    client = null;
}
