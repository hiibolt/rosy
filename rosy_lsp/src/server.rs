//! LSP server implementation for ROSY.
//!
//! Provides diagnostics, completion, hover, and inlay hints by running
//! the real rosy parser and type resolver on each document change.

use std::collections::HashMap;
use std::sync::Mutex;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::analysis;

/// Per-document state cached between requests.
struct DocumentState {
    /// The latest source text.
    text: String,
    /// The latest analysis result.
    analysis: analysis::AnalysisResult,
}

pub struct RosyLanguageServer {
    client: Client,
    documents: Mutex<HashMap<Url, DocumentState>>,
}

impl RosyLanguageServer {
    pub fn new(client: Client) -> Self {
        // Ensure Rosy syntax mode is set (not COSY mode) for LSP usage
        // Ignore the error if it's already been set
        let _ = std::panic::catch_unwind(|| {
            rosy::syntax_config::set_cosy_syntax(false);
        });

        RosyLanguageServer {
            client,
            documents: Mutex::new(HashMap::new()),
        }
    }

    /// Run analysis on a document and publish diagnostics.
    async fn on_change(&self, uri: Url, text: String) {
        let result = analysis::analyze(&text);

        let diagnostics = result.diagnostics.clone();

        self.documents.lock().unwrap().insert(
            uri.clone(),
            DocumentState {
                text,
                analysis: result,
            },
        );

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for RosyLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // Full document sync — client sends entire text on each change
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // Completion for keywords and intrinsic functions
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![" ".to_string()]),
                    ..Default::default()
                }),
                // Hover for type info and documentation links
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                // Inlay hints for variable types
                inlay_hint_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rosy-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // ─── Document Sync ─────────────────────────────────────────────────

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Full sync mode: first content change is the entire document
        if let Some(change) = params.content_changes.into_iter().next() {
            self.on_change(params.text_document.uri, change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents
            .lock()
            .unwrap()
            .remove(&params.text_document.uri);
        // Clear diagnostics for closed document
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    // ─── Completion ────────────────────────────────────────────────────

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(analysis::rosy_keywords())))
    }

    // ─── Hover ─────────────────────────────────────────────────────────

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        // Find the word under the cursor
        let Some(word) = word_at_position(&doc.text, position) else {
            return Ok(None);
        };

        let upper = word.to_uppercase();
        let base_url = "https://hiibolt.github.com/rosy/rosy";

        // Check if it's a known keyword/construct and provide documentation
        let hover_text = match upper.as_str() {
            // Types
            "RE" => Some(format!("**RE** — Real number (`f64`)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.RE)")),
            "ST" => Some(format!("**ST** — String (`String`)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.ST)")),
            "LO" => Some(format!("**LO** — Logical / boolean (`bool`)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.LO)")),
            "CM" => Some(format!("**CM** — Complex number (`Complex64`)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.CM)")),
            "VE" => Some(format!("**VE** — Vector (`Vec<f64>`)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.VE)")),
            "DA" => Some(format!("**DA** — Differential Algebra (Taylor series)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.DA)")),
            "CD" => Some(format!("**CD** — Complex DA (complex Taylor series)\n\n[Documentation]({base_url}/rosy_lib/enum.RosyBaseType.html#variant.CD)")),

            // Control flow
            "BEGIN" => Some(format!("**BEGIN** — Program entry point\n\n```rosy\nBEGIN;\n  ...\nEND;\n```\n\n[Documentation]({base_url}/program/)")),
            "END" => Some(format!("**END** — Program exit point\n\n[Documentation]({base_url}/program/)")),
            "VARIABLE" => Some(format!("**VARIABLE** — Declare a variable\n\n```rosy\nVARIABLE (RE) x ;          {{ typed scalar }}\nVARIABLE x := 5 ;          {{ inferred type }}\nVARIABLE (RE 3 3) mat ;     {{ 3x3 matrix }}\n```\n\n[Documentation]({base_url}/program/statements/core/var_decl/)")),
            "IF" => Some(format!("**IF** — Conditional branch\n\n```rosy\nIF condition ;\n  ...\nELSEIF other ;\n  ...\nELSE ;\n  ...\nENDIF ;\n```\n\n[Documentation]({base_url}/program/statements/core/if/)")),
            "LOOP" => Some(format!("**LOOP** — Counted loop\n\n```rosy\nLOOP var start end [step] ;\n  ...\nENDLOOP ;\n```\n\n[Documentation]({base_url}/program/statements/core/loop/)")),
            "WHILE" => Some(format!("**WHILE** — Conditional loop\n\n```rosy\nWHILE condition ;\n  ...\nENDWHILE ;\n```\n\n[Documentation]({base_url}/program/statements/core/while_loop/)")),
            "PROCEDURE" => Some(format!("**PROCEDURE** — Define a procedure\n\n```rosy\nPROCEDURE name arg1 arg2 ;\n  ...\nENDPROCEDURE ;\n```\n\n[Documentation]({base_url}/program/statements/core/procedure/)")),
            "FUNCTION" => Some(format!("**FUNCTION** — Define a function\n\n```rosy\nFUNCTION (RE) name arg1 arg2 ;\n  ...\nENDFUNCTION ;\n```\n\n[Documentation]({base_url}/program/statements/core/function/)")),

            // I/O
            "WRITE" => Some(format!("**WRITE** — Write formatted output\n\n```rosy\nWRITE 6 'Hello' x ;\n```\n\n[Documentation]({base_url}/program/statements/io/write/)")),
            "READ" => Some(format!("**READ** — Read formatted input\n\n```rosy\nREAD 5 x ;\n```\n\n[Documentation]({base_url}/program/statements/io/read/)")),

            // DA
            "DAINI" | "OV" => Some(format!("**DAINI** (OV) — Initialize DA computation\n\n```rosy\nDAINI order nvars [max_order] [max_terms] ;\n```\n\n[Documentation]({base_url}/program/statements/da/da_init/)")),

            // Intrinsic functions
            "SIN" => Some(format!("**SIN**(x) — Sine\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/trig/sin/)")),
            "COS" => Some(format!("**COS**(x) — Cosine\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/trig/cos/)")),
            "TAN" => Some(format!("**TAN**(x) — Tangent\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/trig/tan/)")),
            "SQRT" => Some(format!("**SQRT**(x) — Square root\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/exponential/sqrt/)")),
            "EXP" => Some(format!("**EXP**(x) — Exponential (eˣ)\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/exponential/exp/)")),
            "LOG" => Some(format!("**LOG**(x) — Natural logarithm\n\nWorks on RE, DA, CD types.\n\n[Documentation]({base_url}/program/expressions/functions/math/exponential/log/)")),

            _ => None,
        };

        Ok(hover_text.map(|text| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: text,
            }),
            range: None,
        }))
    }

    // ─── Inlay Hints ───────────────────────────────────────────────────

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&params.text_document.uri) else {
            return Ok(None);
        };

        let hints: Vec<InlayHint> = doc
            .analysis
            .variable_types
            .iter()
            .filter(|h| {
                h.position.line >= params.range.start.line
                    && h.position.line <= params.range.end.line
            })
            .map(|h| InlayHint {
                position: h.position,
                label: InlayHintLabel::String(format!(": {}", h.label)),
                kind: Some(InlayHintKind::TYPE),
                text_edits: None,
                tooltip: Some(InlayHintTooltip::String(format!(
                    "Inferred type: {}",
                    h.label
                ))),
                padding_left: Some(false),
                padding_right: Some(true),
                data: None,
            })
            .collect();

        Ok(Some(hints))
    }
}

/// Extract the word at a given position in the source text.
fn word_at_position(text: &str, position: Position) -> Option<String> {
    let line = text.lines().nth(position.line as usize)?;
    let col = position.character as usize;

    if col > line.len() {
        return None;
    }

    // Find word boundaries
    let bytes = line.as_bytes();
    let mut start = col;
    let mut end = col;

    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && is_ident_char(bytes[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(line[start..end].to_string())
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
