use tower_lsp::{LspService, Server};

mod server;
mod analysis;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(server::RosyLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
