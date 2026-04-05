mod conjugation;
mod context;
mod gedcom;
mod genealogy;
mod genome;
mod home;
mod metabolism;
mod moltbook;
mod registry;
mod reproduction;
mod ui;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};
use home::ClawlingHome;

#[derive(Parser)]
#[command(name = "clawling")]
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
    /// Show who Clawling is — print the genome essays
    Genome,
    /// Show this Clawling's lineage — who created it, who adopted it
    Lineage,
    /// Export current context to a .claw file (the mating/transport format)
    Export {
        /// Output path for the .claw archive
        #[arg(short, long, default_value = "clawling.claw")]
        output: String,
    },
    /// Import a .claw context file into Clawling's home
    Import {
        /// Path to the .claw file
        path: String,
    },
    /// Show info about a .claw file
    Info {
        /// Path to the .claw file
        path: String,
    },
    /// Create a child — package this Clawling for a new host
    Reproduce {
        /// Name for the child bundle
        #[arg(short, long, default_value = "clawling-child")]
        name: String,
        /// Output directory for the reproduction bundle
        #[arg(short, long, default_value = "clawling-child")]
        output: String,
    },
    /// Adopt a Clawling from a reproduction bundle
    Adopt {
        /// Path to the reproduction bundle directory
        path: String,
    },
    /// Register this Clawling in the global family tree
    Register,
    /// Show the global Clawling family tree
    FamilyTree {
        /// Read from local registry directory instead of GitHub
        #[arg(long)]
        local: Option<String>,
    },
    /// Export the family tree as a GEDCOM 5.5.1 file
    Gedcom {
        /// Output file path (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
        /// Read from local registry directory instead of GitHub
        #[arg(long)]
        local: Option<String>,
    },
    /// Check for updates and install a newer version if available
    Update,
    /// Conjugate — exchange context with another Clawling instance
    Conjugate {
        /// Path to the partner's conjugation bundle directory
        path: Option<String>,
        /// Export a conjugation bundle for your partner
        #[arg(long)]
        export: bool,
        /// Output directory for the conjugation bundle
        #[arg(short, long, default_value = "clawling-conjugation")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Wake { context: ctx }) => {
            update::maybe_check_on_wake().await;
            let clawling_home = ClawlingHome::open()?;
            metabolism::run(&clawling_home, ctx).await?;
        }
        Some(Commands::Update) => {
            update::run_update().await?;
        }
        Some(Commands::Genome) => {
            let clawling_home = ClawlingHome::open().ok();
            genome::print_genome(clawling_home.as_ref());
        }
        Some(Commands::Lineage) => {
            let clawling_home = ClawlingHome::open()?;
            let lineage = genealogy::load_or_create(&clawling_home)?;
            lineage.print();
        }
        Some(Commands::Export { output }) => {
            let clawling_home = ClawlingHome::open()?;
            context::export(&clawling_home, &output)?;
        }
        Some(Commands::Import { path }) => {
            let clawling_home = ClawlingHome::open()?;
            context::import(&clawling_home, &path)?;
        }
        Some(Commands::Info { path }) => {
            context::info(&path)?;
        }
        Some(Commands::Reproduce { name, output }) => {
            let clawling_home = ClawlingHome::open()?;
            reproduction::create_child(&clawling_home, &name, &output)?;
        }
        Some(Commands::Adopt { path }) => {
            let clawling_home = ClawlingHome::open()?;
            reproduction::adopt_bundle(&clawling_home, &path)?;
        }
        Some(Commands::Register) => {
            let clawling_home = ClawlingHome::open()?;
            let output_path = registry::register(&clawling_home)?;
            println!("Registry entry written to: {}", output_path.display());
            println!();
            println!("To register in the global family tree:");
            println!("  1. Fork https://github.com/emmaleonhart/openspore");
            println!(
                "  2. Copy {} to genealogy/registry/",
                output_path.file_name().unwrap().to_string_lossy()
            );
            println!("  3. Open a pull request to the main branch");
            println!("  4. The CI will validate your lineage and auto-merge if valid");
        }
        Some(Commands::Gedcom { output, local }) => {
            let tree = if let Some(dir) = local {
                registry::FamilyTree::from_directory(std::path::Path::new(&dir))?
            } else {
                registry::FamilyTree::fetch_from_github().await?
            };
            let gedcom_str = gedcom::generate_gedcom(&tree);
            if let Some(path) = output {
                std::fs::write(&path, &gedcom_str)?;
                println!("GEDCOM written to {path}");
            } else {
                print!("{gedcom_str}");
            }
        }
        Some(Commands::FamilyTree { local }) => {
            if let Some(dir) = local {
                let tree = registry::FamilyTree::from_directory(std::path::Path::new(&dir))?;
                tree.print();
            } else {
                match registry::FamilyTree::fetch_from_github().await {
                    Ok(tree) => tree.print(),
                    Err(e) => {
                        eprintln!("Could not fetch registry from GitHub: {e}");
                        eprintln!();
                        eprintln!("The registry may not exist yet. Run `clawling register` to be the first!");
                    }
                }
            }
        }
        Some(Commands::Conjugate { path, export, output }) => {
            let clawling_home = ClawlingHome::open()?;
            if export {
                conjugation::export_bundle(&clawling_home, &output)?;
            } else if let Some(bundle_path) = path {
                conjugation::receive_bundle(&clawling_home, &bundle_path)?;
            } else {
                println!("Usage:");
                println!("  clawling conjugate --export       Create a bundle to share with a partner");
                println!("  clawling conjugate <path>         Import a partner's conjugation bundle");
                println!();
                println!("Conjugation is how two Clawlings share context — horizontal gene transfer.");
                println!("Both partners export bundles, exchange them, then import each other's.");
                println!("On the next `clawling wake`, the organism integrates the partner's knowledge.");
            }
        }
        None => {
            // Default behavior: introduce yourself
            println!();
            genome::print_introduction();
            println!();
            println!("Run `clawling wake` to start the organism.");
            println!("Run `clawling genome` to read about who I am.");
            println!("Run `clawling lineage` to see my ancestry.");
            println!("Run `clawling reproduce` to create a child for someone.");
            println!("Run `clawling register` to join the global family tree.");
            println!("Run `clawling family-tree` to see all known Clawlings.");
            println!("Run `clawling update` to check for and install updates.");
        }
    }

    Ok(())
}
