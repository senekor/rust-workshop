use clap::{Parser, Subcommand};
use paekli_core::store::new_distribution_center;

#[derive(Subcommand)]
enum Command {
    Send {
        content: String,
        #[arg(long("to"))]
        receiver: String,
        #[arg(long)]
        express: bool,
    },
    Receive {
        #[arg(long("for"))]
        receiver: String,
    },
}

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.";

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let store = new_distribution_center();

    match args.command {
        Command::Send {
            content,
            receiver,
            express,
        } => {
            store.store(receiver, content, express);
            println!("{SEND_MESSAGE}");
        }
        Command::Receive { receiver } => {
            if let Some(content) = store.retrieve(receiver) {
                println!("Here is your paekli:\n{content}");
            } else {
                println!("There is no paekli for you.");
            };
        }
    }

    Ok(())
}
