#![allow(unused_imports)]
#![allow(unused_variables)]
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct JsonServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for JsonServer {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: None,
                text_document_sync: None,
                ..Default::default()
            },
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    /* fn text_document_did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(Message {
                message: format!("Text document {} open", params.text_document.uri),
                ty: MessageType::Info,
            })
            .unwrap();
    }
    */

    // Implement other required methods here...
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, sock) = LspService::new(|client| JsonServer { client });
    Server::new(stdin, stdout, sock).serve(service).await;
}
