#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    dashmap::DashMap,
    ropey::Rope,
    serde::{Deserialize, Serialize},
    serde_json::Value,
    tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server},
};

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
    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        dbg!("file opened");
        /* self.client.log_message(Message {
            message: format!("Text document {} open", params.text_document.uri),
            ty: MessageType::Info,
            language_id: ???
        });
        */
    }

    // Implement other required methods here...
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, sock) = LspService::new(|client| JsonServer { client });
    Server::new(stdin, stdout, sock).serve(service).await;
}
impl JsonServer {}
