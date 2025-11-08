use lit_recovery::args::Commands;
use lit_recovery::error::{Error, RecoveryResult};
use lit_recovery::shares::{
    COLUMN_CURVE, COLUMN_DECRYPTION_KEY_SHARE, COLUMN_ENCRYPTION_KEY, COLUMN_SESSION_ID,
    COLUMN_SUBNET_ID, COLUMN_URL,
};
use path_clean::clean;
use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::{
    Completer, Context, Helper, Validator,
    completion::FilenameCompleter,
    highlight::{CmdKind, Highlighter, MatchingBracketHighlighter},
    hint::{Hint, Hinter},
    validate::MatchingBracketValidator,
};
use std::collections::{HashMap, HashSet};
use std::iter;

pub struct Parser<'a> {
    s: &'a str,
    it: iter::Peekable<std::str::CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, it: s.char_indices().peekable() }
    }

    pub fn parse_command(s: &'a str) -> RecoveryResult<Commands> {
        let mut parser = Self::new(s);

        match parser.command()? {
            None => Err(Error::General("no command found".to_string())),
            Some(command) => match command {
                "register" => Ok(Commands::RegisterToRecoverContract {}),
                "download" => Ok(Commands::DownloadShare {}),
                "upload-pub-keys" => Ok(Commands::UploadPublicKey {}),
                "list" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::ListShareDetails {
                        session_id: map.get(COLUMN_SESSION_ID).map(|s| s.to_string()),
                        encryption_key: map.get(COLUMN_ENCRYPTION_KEY).map(|s| s.to_string()),
                        curve: map.get(COLUMN_CURVE).map(|s| s.to_string()),
                        subnet_id: map.get(COLUMN_SUBNET_ID).map(|s| s.to_string()),
                        url: map.get(COLUMN_URL).map(|s| s.to_string()),
                    })
                }
                "delete" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::DeleteShare {
                        session_id: map.get(COLUMN_SESSION_ID).map(|s| s.to_string()),
                        encryption_key: map.get(COLUMN_ENCRYPTION_KEY).map(|s| s.to_string()),
                        curve: map.get(COLUMN_CURVE).map(|s| s.to_string()),
                        subnet_id: map.get(COLUMN_SUBNET_ID).map(|s| s.to_string()),
                        url: map.get(COLUMN_URL).map(|s| s.to_string()),
                    })
                }
                "insert-share" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }

                    Ok(Commands::InsertShare {
                        session_id: map
                            .get(COLUMN_SESSION_ID)
                            .map(|s| s.to_string())
                            .unwrap_or("test_session_id".to_string()),
                        encryption_key: map
                            .get(COLUMN_ENCRYPTION_KEY)
                            .map(|s| s.to_string())
                            .ok_or(Error::General(
                                "missing encryption_key parameter".to_string(),
                            ))?,
                        decryption_key_share: map
                            .get(COLUMN_DECRYPTION_KEY_SHARE)
                            .map(|s| s.to_string())
                            .ok_or(Error::General(
                                "missing decryption_key_share parameter".to_string(),
                            ))?,
                        subnet_id: map
                            .get(COLUMN_SUBNET_ID)
                            .map(|s| s.to_string())
                            .unwrap_or("test_subnet_id".to_string()),
                        curve: map
                            .get(COLUMN_CURVE)
                            .map(|s| s.to_string())
                            .ok_or(Error::General("missing curve parameter".to_string()))?,
                        url: map
                            .get(COLUMN_URL)
                            .map(|s| s.to_string())
                            .unwrap_or("test_url".to_string()),
                    })
                }
                "import" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::ImportSharesFromFile {
                        file: map
                            .get("file")
                            .map(clean)
                            .ok_or(Error::General("missing file parameter".to_string()))?,
                        import_password: map.get("password").map(|s| s.to_string()),
                    })
                }
                "export" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::ExportSharesToFile {
                        file: map
                            .get("file")
                            .map(clean)
                            .ok_or(Error::General("missing file parameter".to_string()))?,
                        export_password: map.get("password").map(|s| s.to_string()),
                    })
                }
                "upload" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::UploadDecryptionShare {
                        key_type: map
                            .get("key_type")
                            .map(|s| s.to_string())
                            .ok_or(Error::General("missing key_type parameter".to_string()))?,
                        ciphertext_file: map.get("ciphertext_file").map(clean).ok_or(
                            Error::General("missing ciphertext_file parameter".to_string()),
                        )?,
                        encryption_key: map.get("encryption_key").map(|s| s.to_string()).ok_or(
                            Error::General("missing encryption_key parameter".to_string()),
                        )?,
                    })
                }
                "recover" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::Recover {
                        directory: map
                            .get("directory")
                            .map(clean)
                            .ok_or(Error::General("missing directory parameter".to_string()))?,
                        session_id: map
                            .get("session_id")
                            .map(|s| s.to_string())
                            .ok_or(Error::General("missing session_id parameter".to_string()))?,
                    })
                }
                "mnemonic" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::Mnemonic {
                        phrase: map
                            .get("phrase")
                            .map(|s| s.to_string())
                            .ok_or(Error::General("missing mnemonic parameter".to_string()))?,
                    })
                }
                "contract-resolver" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }

                    let address =
                        map.get("address").map(|s| s.to_string()).ok_or(Error::General(
                            "missing address parameter, use address=[contract address".to_string(),
                        ))?;

                    Ok(Commands::ContractResolver { address })
                }
                "config" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }

                    let address =
                        map.get("address").map(|s| s.to_string()).ok_or(Error::General(
                            "missing address parameter use address=[address]".to_string(),
                        ))?;

                    let rpc_url =
                        map.get("rpc_url").map(|s| s.to_string()).ok_or(Error::General(
                            "missing rpc url parameter use rpc_url=[rpc url]".to_string(),
                        ))?;

                    let chain_id = map
                        .get("chain_id")
                        .ok_or(Error::General(
                            "missing chain id parameter use chain_id=[chain id]".to_string(),
                        ))?
                        .parse::<u64>()
                        .map_err(|_| Error::General("Cannot parse chain_id as u64".to_string()))?;

                    let env =
                        map.get("env")
                            .ok_or(Error::General(
                                "missing env parameter use env=[enviorment]; 0 for dev, 1 for staging and 2 for prod".to_string(),
                            ))?
                        .parse::<u8>().map_err(|_| Error::General("Cannot parse env as u8".to_string()))?;
                    Ok(Commands::SetConfig { address, rpc_url, chain_id, env })
                }
                "get-node-status" => Ok(Commands::GetNodeStatus {}),
                "decrypt-share" => {
                    let mut map = HashMap::new();
                    while let Some((key, value)) = parser.parameter()? {
                        map.insert(key, value);
                    }
                    Ok(Commands::DecryptShare {
                        key_type: map
                            .get("key_type")
                            .map(|s| s.to_string())
                            .ok_or(Error::General("missing key_type parameter".to_string()))?,
                        ciphertext_file: map.get("ciphertext_file").map(clean).ok_or(
                            Error::General("missing ciphertext_file parameter".to_string()),
                        )?,
                        share_file: map.get("share_file").map(clean),
                        output_share_file: map.get("output_share_file").map(clean).ok_or(
                            Error::General("missing output_share_file parameter".to_string()),
                        )?,
                        encryption_key: map.get("encryption_key").map(|s| s.to_string()).ok_or(
                            Error::General("missing encryption_key parameter".to_string()),
                        )?,
                    })
                }
                "info" => Ok(Commands::Info),
                _ => Err(Error::General(format!("unknown command `{}`", command))),
            },
        }
    }

    pub fn skip_ws(&mut self) {
        self.take_while(char::is_whitespace);
    }

    pub fn take_while<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = match self.it.peek() {
            Some(&(i, _)) => i,
            None => return "",
        };

        loop {
            match self.it.peek() {
                Some(&(_, c)) if f(c) => {
                    self.it.next();
                }
                Some(&(i, _)) => return &self.s[start..i],
                None => return &self.s[start..],
            }
        }
    }

    pub fn consume(&mut self, target: char) -> RecoveryResult<()> {
        match self.it.next() {
            Some((_, c)) if c == target => Ok(()),
            Some((i, c)) => Err(Error::General(format!(
                "unexpected character at byte {}: expected `{}` but got `{}`",
                i, target, c
            ))),
            None => Err(Error::General("unexpected EOF".to_string())),
        }
    }

    pub fn consume_if(&mut self, target: char) -> bool {
        match self.it.peek() {
            Some(&(_, c)) if c == target => {
                self.it.next();
                true
            }
            _ => false,
        }
    }

    pub fn keyword(&mut self) -> Option<&'a str> {
        let s = self.take_while(|c| !matches!(c, c if c.is_whitespace() || c == '='));

        if s.is_empty() { None } else { Some(s) }
    }

    pub fn value(&mut self) -> RecoveryResult<String> {
        let value = if self.consume_if('\'') {
            let value = self.quoted_value()?;
            self.consume('\'')?;
            value
        } else if self.consume_if('"') {
            let value = self.double_quoted_value()?;
            self.consume('"')?;
            value
        } else {
            self.simple_value()?
        };

        Ok(value)
    }

    pub fn simple_value(&mut self) -> RecoveryResult<String> {
        let mut value = String::new();

        while let Some(&(_, c)) = self.it.peek() {
            if c.is_whitespace() {
                break;
            }

            self.it.next();
            if c == '\\' {
                if let Some((_, c2)) = self.it.next() {
                    value.push(c2);
                }
            } else {
                value.push(c);
            }
        }

        if value.is_empty() {
            return Err(Error::General("unexpected EOF".to_string()));
        }

        Ok(value)
    }

    pub fn quoted_value(&mut self) -> RecoveryResult<String> {
        let mut value = String::new();

        while let Some(&(_, c)) = self.it.peek() {
            if c == '\'' {
                return Ok(value);
            }

            self.it.next();
            if c == '\\' {
                if let Some((_, c2)) = self.it.next() {
                    value.push(c2);
                }
            } else {
                value.push(c);
            }
        }

        Err(Error::General("unterminated quoted parameter value".to_string()))
    }

    pub fn double_quoted_value(&mut self) -> RecoveryResult<String> {
        let mut value = String::new();

        while let Some(&(_, c)) = self.it.peek() {
            if c == '"' {
                return Ok(value);
            }

            self.it.next();
            if c == '\\' {
                if let Some((_, c2)) = self.it.next() {
                    value.push(c2);
                }
            } else {
                value.push(c);
            }
        }

        Err(Error::General("unterminated double quoted parameter value".to_string()))
    }

    pub fn command(&mut self) -> RecoveryResult<Option<&'a str>> {
        self.skip_ws();
        let keyword = match self.keyword() {
            Some(keyword) => keyword,
            None => return Ok(None),
        };
        Ok(Some(keyword))
    }

    pub fn parameter(&mut self) -> RecoveryResult<Option<(&'a str, String)>> {
        self.skip_ws();
        let keyword = match self.keyword() {
            Some(keyword) => keyword,
            None => return Ok(None),
        };
        self.skip_ws();
        self.consume('=')?;
        self.skip_ws();
        let value = self.value()?;

        Ok(Some((keyword, value)))
    }
}

#[derive(Helper, rustyline::Hinter, Completer, Validator)]
pub struct ReplHelper {
    #[rustyline(Completer)]
    pub completer: RecoveryHinter,
    pub highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    pub validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    pub hinter: RecoveryHinter,
    pub colored_prompt: String,
}

impl Highlighter for ReplHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self, prompt: &'p str, default: bool,
    ) -> std::borrow::Cow<'b, str> {
        if default {
            std::borrow::Cow::Borrowed(&self.colored_prompt)
        } else {
            std::borrow::Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

impl Default for ReplHelper {
    fn default() -> Self {
        Self {
            completer: RecoveryHinter::default(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: RecoveryHinter::default(),
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        }
    }
}

pub struct RecoveryHinter {
    hints: HashSet<CommandHint>,
    file_completer: FilenameCompleter,
}

impl Hinter for RecoveryHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(
                |hint| {
                    if hint.display.starts_with(line) { Some(hint.suffix(pos)) } else { None }
                },
            )
            .next()
    }
}

impl Completer for RecoveryHinter {
    type Candidate = Pair;

    fn complete(
        &self, line: &str, pos: usize, ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (start, mut candidates) = self.file_completer.complete(line, pos, ctx)?;
        let prefix = &line[..pos];
        let mut additional = self
            .hints
            .iter()
            .filter_map(|h| if h.display.starts_with(prefix) { Some(h.into()) } else { None })
            .collect::<Vec<Pair>>();
        candidates.append(&mut additional);
        #[allow(clippy::unnecessary_sort_by)]
        candidates.sort_by(|a, b| a.display().cmp(b.display()));
        Ok((start, candidates))
    }
}

impl Default for RecoveryHinter {
    fn default() -> Self {
        let mut set = HashSet::with_capacity(50);
        set.insert(CommandHint::new("register", "register"));
        set.insert(CommandHint::new("download", "download"));
        set.insert(CommandHint::new("upload-pub-keys", "upload-pub-keys"));
        set.insert(CommandHint::new("list", "list"));
        set.insert(CommandHint::new("delete", "delete"));
        set.insert(CommandHint::new("import", "import"));
        set.insert(CommandHint::new("insert-share", "insert-share"));
        set.insert(CommandHint::new("export", "export"));
        set.insert(CommandHint::new("upload", "upload"));
        set.insert(CommandHint::new("recover", "recover"));
        set.insert(CommandHint::new("mnemonic", "mnemonic"));
        set.insert(CommandHint::new("contract-resolver", "contract-resolver"));
        set.insert(CommandHint::new("help", "help"));
        set.insert(CommandHint::new("quit", "quit"));
        set.insert(CommandHint::new("config", "config"));
        set.insert(CommandHint::new("decrypt-share", "decrypt-share"));
        set.insert(CommandHint::new("get-node-status", "get-node-status"));
        set.insert(CommandHint::new("session_id", "session_id"));
        set.insert(CommandHint::new("encryption_key", "encryption_key"));
        set.insert(CommandHint::new("curve", "curve"));
        set.insert(CommandHint::new("subnet_id", "subnet_id"));
        set.insert(CommandHint::new("url", "url"));
        set.insert(CommandHint::new("file", "file"));
        set.insert(CommandHint::new("password", "password"));
        set.insert(CommandHint::new("key_type", "key_type"));
        set.insert(CommandHint::new("ciphertext_file", "ciphertext_file"));
        set.insert(CommandHint::new("phrase", "phrase"));
        set.insert(CommandHint::new("address", "address"));
        set.insert(CommandHint::new("rpc_url", "rpc_url"));
        set.insert(CommandHint::new("chain_id", "chain_id"));
        set.insert(CommandHint::new("env", "env"));
        set.insert(CommandHint::new("merge-decryption-shares", "merge-decryption-shares"));
        set.insert(CommandHint::new("info", "info"));
        set.insert(CommandHint::new("session_id", "session_id"));
        Self { hints: set, file_completer: FilenameCompleter::new() }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct CommandHint {
    pub display: String,
    pub complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to < self.display.len() {
            Some(&self.display[self.complete_up_to..])
        } else {
            None
        }
    }
}

impl From<CommandHint> for Pair {
    fn from(hint: CommandHint) -> Self {
        Pair::from(&hint)
    }
}

impl From<&CommandHint> for Pair {
    fn from(hint: &CommandHint) -> Self {
        Pair { display: hint.display.clone(), replacement: hint.display.clone() }
    }
}

impl CommandHint {
    fn new(display: &str, complete_up_to: &str) -> Self {
        assert!(display.starts_with(complete_up_to));
        Self { display: display.to_string(), complete_up_to: complete_up_to.len() }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}
