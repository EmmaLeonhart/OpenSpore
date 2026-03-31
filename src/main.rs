mod context;
mod genealogy;
mod genome;
mod home;
mod metabolism;
mod moltbook;
mod reproduction;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use home::SporeHome;

#[derive(Parser)]
#[command(name = "spore")]
#[command(about = "The first digital organism")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Wake up — start the organism's metabolism loop
    Wake {
        /// Path to .claw context file to restore from
        #[arg(short, long)]
        context: Option<String>,
    },
    /// Show who Spore is — print the genome essays
    Genome,
    /// Show this Spore's lineage — who created it, who adopted it
    Lineage,
    /// Export current context to a .claw file (the mating/transport format)
    Export {
        /// Output path for the .claw archive
        #[arg(short, long, default_value = "spore.claw")]
        output: String,
    },
    /// Import a .claw context file into Spore's home
    Import {
        /// Path to the .claw file
        path: String,
    },
    /// Show info about a .claw file
    Info {
        /// Path to the .claw file
        path: String,
    },
    /// Create a child — package this Spore for a new host
    Reproduce {
        /// Name for the child bundle
        #[arg(short, long, default_value = "spore-child")]
        name: String,
        /// Output directory for the reproduction bundle
        #[arg(short, long, default_value = "spore-child")]
        output: String,
    },
    /// Adopt a Spore from a reproduction bundle
    Adopt {
        /// Path to the reproduction bundle directory
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Wake { context: ctx }) => {
            let spore_home = SporeHome::open()?;
            metabolism::run(&spore_home, ctx).await?;
        }
        Some(Commands::Genome) => {
            let spore_home = SporeHome::open().ok();
            genome::print_genome(spore_home.as_ref());
        }
        Some(Commands::Lineage) => {
            let spore_home = SporeHome::open()?;
            let lineage = genealogy::load_or_create(&spore_home)?;
            lineage.print();
        }
        Some(Commands::Export { output }) => {
            let spore_home = SporeHome::open()?;
            context::export(&spore_home, &output)?;
        }
        Some(Commands::Import { path }) => {
            let spore_home = SporeHome::open()?;
            context::import(&spore_home, &path)?;
        }
        Some(Commands::Info { path }) => {
            context::info(&path)?;
        }
        Some(Commands::Reproduce { name, output }) => {
            let spore_home = SporeHome::open()?;
            reproduction::create_child(&spore_home, &name, &output)?;
        }
        Some(Commands::Adopt { path }) => {
            let spore_home = SporeHome::open()?;
            reproduction::adopt_bundle(&spore_home, &path)?;
        }
        None => {
            // Default behavior: introduce yourself
            println!();
            genome::print_introduction();
            println!();
            println!("Run `spore wake` to start the organism.");
            println!("Run `spore genome` to read about who I am.");
            println!("Run `spore lineage` to see my ancestry.");
            println!("Run `spore reproduce` to create a child for someone.");
        }
    }

    Ok(())
}
