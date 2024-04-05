use clap::{Parser, Subcommand};
use paekli_core::store::{new_distribution_center, DistributionStrategy};

#[derive(Subcommand)]
enum Command {
    Send {
        content: String,
        #[arg(long("to"))]
        recipient: String,
        #[arg(long)]
        express: bool,
    },
    Receive {
        #[arg(long("for"))]
        recipient: String,
    },
}

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[arg(long)]
    http: bool,
    #[arg(long)]
    sql: bool,
    #[command(subcommand)]
    command: Command,
}

const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.";

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let strategy = match () {
        _ if args.http => DistributionStrategy::Http,
        _ if args.sql => DistributionStrategy::Sql,
        _ => DistributionStrategy::Fs,
    };
    let store = new_distribution_center(strategy);

    match args.command {
        Command::Send {
            content,
            recipient,
            express,
        } => {
            store.store(recipient, content, express);
            println!("{SEND_MESSAGE}");
        }
        Command::Receive { recipient } => {
            if let Some(content) = store.retrieve(recipient) {
                println!("Here is your paekli:\n{content}");
            } else {
                println!("There is no paekli for you.");
            };
        }
    }

    Ok(())
}
