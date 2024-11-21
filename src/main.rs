#![allow(unused_imports)]
use tokio::*;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService};

#[derive(Debug)]
struct JsonServer {
    client: Client,
}

// #[async_trait]
impl LanguageServer for JsonServer {
    fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    fn initialized(&self, _params: InitializedParams) {}

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use io::stdin;
    use io::stdout;

    async fn serve_lsp_service() -> Result<(), Box<dyn std::error::Error>> {
        let stdin = stdin();
        let stdout = stdout();

        let (mut service, mut sock) = LspService::new(|client| JsonServer { client });
        service.run(sock).await?;

        Ok(())
    }

    tokio::runtime::Runtime::new()?.block_on(serve_lsp_service())
}
