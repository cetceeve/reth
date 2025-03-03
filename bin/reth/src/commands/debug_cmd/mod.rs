//! `reth debug` command. Collection of various debugging routines.

use clap::{Parser, Subcommand};

use crate::runner::CliContext;

mod build_block;
mod execution;
mod in_memory_merkle;
mod merkle;

/// `reth debug` command
#[derive(Debug, Parser)]
pub struct Command {
    #[clap(subcommand)]
    command: Subcommands,
}

/// `reth debug` subcommands
#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Debug the roundtrip execution of blocks as well as the generated data.
    Execution(execution::Command),
    /// Debug the clean & incremental state root calculations.
    Merkle(merkle::Command),
    /// Debug in-memory state root calculation.
    InMemoryMerkle(in_memory_merkle::Command),
    /// Debug block building.
    BuildBlock(build_block::Command),
}

impl Command {
    /// Execute `debug` command
    pub async fn execute(self, ctx: CliContext) -> eyre::Result<()> {
        match self.command {
            Subcommands::Execution(command) => command.execute(ctx).await,
            Subcommands::Merkle(command) => command.execute(ctx).await,
            Subcommands::InMemoryMerkle(command) => command.execute(ctx).await,
            Subcommands::BuildBlock(command) => command.execute(ctx).await,
        }
    }
}
