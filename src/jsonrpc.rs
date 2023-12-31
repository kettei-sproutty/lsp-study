use serde::{Serialize, Deserialize};

/// Represents a Language Server Protocol method.
/// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
#[derive(Serialize)]
pub enum LspMethod {
    /// Initialize the language server.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize)
    Initialize,
    /// Shutdown the language server.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#shutdown)
    Shutdown,
    /// Notify the language server that a document has changed.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_didChange)
    DidChange,
    /// Notify the language server that a document has been opened.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_didOpen)
    DidOpen,
    /// Notify the language server that a document has been closed.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_didClose)
    DidClose,
    /// Request the language server to provide hover information for a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_hover)
    Hover,
    /// Request the language server to provide completion suggestions for a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_completion)
    Completion,
    /// Request the language server to provide signature help for a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_signatureHelp)
    SignatureHelp,
    /// Request the language server to provide the definition of a symbol at a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_definition)
    Definition,
    /// Request the language server to provide references to a symbol at a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_references)
    References,
    /// Request the language server to provide the document symbols for a given document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentSymbol)
    DocumentSymbol,
    /// Request the language server to provide the document highlights for a given document and position.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentHighlights)
    DocumentHighlights,
    /// Request the language server to provide the formatting options for a given document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_formatting)
    DocumentFormatting,
    /// Request the language server to format a given range in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_rangeFormatting)
    RangeFormatting,
    /// Request the language server to format the text inserted at a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_onTypeFormatting)
    OnTypeFormatting,
    /// Request the language server to provide code actions for a given document and position.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction)
    CodeAction,
    /// Request the language server to provide symbols for a given workspace.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_symbol)
    WorkspaceSymbol,
    /// Request the language server to provide references to a symbol in a given workspace.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_references)
    WorkspaceReferences,
    /// Request the language server to rename a symbol at a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_rename)
    Rename,
    /// Request the language server to prepare for a rename operation at a given position in a document.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_prepareRename)
    PrepareRename,
    /// Request the language server to execute a command.
    /// [See the LSP specification for more details.](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_executeCommand)
    ExecuteCommand,
}

impl LspMethod {
    /// Converts the `LspMethod` variant to a string.
    pub fn to_string(&self) -> &'static str {
        match self {
            LspMethod::Initialize => "initialize",
            LspMethod::Shutdown => "shutdown",
            LspMethod::DidChange => "textDocument/didChange",
            LspMethod::DidOpen => "textDocument/didOpen",
            LspMethod::DidClose => "textDocument/didClose",
            LspMethod::Hover => "textDocument/hover",
            LspMethod::Completion => "textDocument/completion",
            LspMethod::SignatureHelp => "textDocument/signatureHelp",
            LspMethod::Definition => "textDocument/definition",
            LspMethod::References => "textDocument/references",
            LspMethod::DocumentSymbol => "textDocument/documentSymbol",
            LspMethod::DocumentHighlights => "textDocument/documentHighlights",
            LspMethod::DocumentFormatting => "textDocument/formatting",
            LspMethod::RangeFormatting => "textDocument/rangeFormatting",
            LspMethod::OnTypeFormatting => "textDocument/onTypeFormatting",
            LspMethod::CodeAction => "textDocument/codeAction",
            LspMethod::Rename => "textDocument/rename",
            LspMethod::PrepareRename => "textDocument/prepareRename",
            LspMethod::WorkspaceSymbol => "workspace/symbol",
            LspMethod::WorkspaceReferences => "workspace/references",
            LspMethod::ExecuteCommand => "workspace/executeCommand",
        }
    }

    /// Converts the string representation of an LSP method to the corresponding `LspMethod` variant.
    pub fn from_str(s: &str) -> Option<LspMethod> {
        match s {
            "initialize" => Some(LspMethod::Initialize),
            "shutdown" => Some(LspMethod::Shutdown),
            "textDocument/didChange" => Some(LspMethod::DidChange),
            "textDocument/didOpen" => Some(LspMethod::DidOpen),
            "textDocument/didClose" => Some(LspMethod::DidClose),
            "textDocument/hover" => Some(LspMethod::Hover),
            "textDocument/completion" => Some(LspMethod::Completion),
            "textDocument/signatureHelp" => Some(LspMethod::SignatureHelp),
            "textDocument/definition" => Some(LspMethod::Definition),
            "textDocument/references" => Some(LspMethod::References),
            "textDocument/documentSymbol" => Some(LspMethod::DocumentSymbol),
            "textDocument/documentHighlights" => Some(LspMethod::DocumentHighlights),
            "textDocument/formatting" => Some(LspMethod::DocumentFormatting),
            "textDocument/rangeFormatting" => Some(LspMethod::RangeFormatting),
            "textDocument/onTypeFormatting" => Some(LspMethod::OnTypeFormatting),
            "textDocument/codeAction" => Some(LspMethod::CodeAction),
            "workspace/symbol" => Some(LspMethod::WorkspaceSymbol),
            "workspace/references" => Some(LspMethod::WorkspaceReferences),
            "textDocument/rename" => Some(LspMethod::Rename),
            "textDocument/prepareRename" => Some(LspMethod::PrepareRename),
            "workspace/executeCommand" => Some(LspMethod::ExecuteCommand),
            _ => None,
        }
    }
}

/// Represents a JSON-RPC (Remote Procedure Call) request.
#[derive(Serialize)]
pub struct JsonRpcRequest<'a, T: Serialize> {
    /// Specifies the version of the JSON-RPC protocol. It must always be set to `"2.0"`.
    pub jsonrpc: &'a str,
    /// Specifies the name of the method to be invoked on the remote server.
    pub method: &'a str,
    /// Contains the parameters to be passed to the method specified in the `method` field.
    pub params: Vec<T>,
    /// An identifier for the request. This is used to match responses to their corresponding requests.
    pub id: u64,
}

/// Represents a JSON-RPC response.
#[derive(Deserialize)]
pub struct JsonRpcResponse<'a, T: Serialize> {
    /// Specifies the version of the JSON-RPC protocol. It should always be "2.0".
    pub jsonrpc: &'a str,
    /// Contains the result of a successful JSON-RPC request, if applicable.
    pub result: Option<T>,
    /// Contains the error details in case the JSON-RPC request encounters an error.
    pub error: Option<JsonRpcError>,
    /// An identifier that associates the response with a specific request.
    pub id: u64,
}


/// Represents a JSON-RPC error response.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// A numeric error code that indicates the type of error.
    pub code: i32,
    /// A short, human-readable error message providing additional information about the error.
    pub message: String,
    /// An optional data field that can hold additional error information, typically structured as a JSON object.
    pub data: Option<serde_json::Value>,
}
