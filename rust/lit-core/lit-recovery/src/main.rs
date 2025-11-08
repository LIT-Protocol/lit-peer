mod repl;

use clap::Parser;
use colored::Colorize;
use lit_recovery::LitRecovery;
use lit_recovery::args::*;
use lit_recovery::error::*;

#[tokio::main]
async fn main() -> RecoveryResult<()> {
    // Print the version of the tool - env macro captures the value at the time of compilation
    println!("Lit recovery tool version: {}", env!("CARGO_PKG_VERSION").green());

    let args = Args::parse();
    let recovery =
        LitRecovery::new(args.file.clone(), args.password.clone(), args.shares_db.clone(), None)
            .await?;

    match args.command {
        Some(command) => recovery.command(command).await,
        None => run_repl(recovery).await,
    }
}

async fn run_repl(recovery: LitRecovery) -> RecoveryResult<()> {
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .build();
    let h = repl::ReplHelper::default();
    let mut rl = rustyline::Editor::with_config(config)?;
    rl.set_helper(Some(h));

    let mut input;
    print_help();

    loop {
        rl.helper_mut().expect("No helper").colored_prompt = "recovery> ".green().to_string();
        let readline = rl.readline("recovery> ");
        match readline {
            Ok(line) => {
                input = line;
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }
        rl.add_history_entry(input.as_str())?;
        // execute command
        let mut parser = repl::Parser::new(input.as_str());
        parser.skip_ws();
        match parser.simple_value() {
            Err(_) => {}
            Ok(word) => match word.as_str() {
                "register" | "download" | "upload-pub-keys" | "list" | "delete" | "import"
                | "export" | "insert-share" | "upload" | "mnemonic" | "contract-resolver"
                | "config" | "recover" | "info" | "get-node-status" => {
                    match repl::Parser::parse_command(input.as_str()) {
                        Err(e) => eprintln!("{}", e),
                        Ok(command) => {
                            if let Err(e) = recovery.command(command).await {
                                eprintln!("{}", e);
                            }
                        }
                    }
                }
                "help" => {
                    print_help();
                }
                "quit" => {
                    break;
                }
                _ => {
                    eprintln!("Unknown command: {}", word);
                }
            },
        }
    }
    Ok(())
}

fn print_help() {
    println!("Commands:");
    println!("register");
    println!("download");
    println!("upload-pub-keys");
    println!(
        "list [session_id=STRING] [encryption_key=STRING] [curve=STRING] [subnet_id=STRING] [url=STRING] [participant_id=INTEGER]"
    );
    println!(
        "delete [session_id=STRING] [encryption_key=STRING] [curve=STRING] [subnet_id=STRING] [url=STRING] [participant_id=INTEGER]"
    );
    println!("import file=PATH [password=STRING]");
    println!(
        "insert-share participant_id=INTEGER [session_id=STRING] encryption_key=STRING decryption_key_share=STRING [subnet_id=STRING] curve=STRING [url=STRING]"
    );
    println!("export file=PATH [password=STRING]");
    println!("upload key_type=STRING ciphertext_file=PATH encryption_key=STRING");
    println!("recover directory=PATH session_id=STRING");
    println!("mnemonic phrase=STRING");
    println!("contract-resolver [address=STRING]");
    println!("config [address=STRING] [rpc_url=STRING] [chain_id=INTEGER] [env=INTEGER]");
    println!("decrypt-share share_file=PATH");
    println!("get-node-status");
    println!("info");
    println!("quit");
}
