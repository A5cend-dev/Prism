//! `prism diff` — Show state diff (before/after) for a transaction.

use clap::Args;
use prism_core::types::config::NetworkConfig;
use crate::output::renderers;

#[derive(Args)]
pub struct DiffArgs {
    /// Transaction hash to diff.
    pub tx_hash: String,
}

pub async fn run(
    args: DiffArgs,
    network: &NetworkConfig,
    output_format: &str
) -> anyhow::Result<()> {
    let progress = indicatif::ProgressBar::new_spinner();
    progress.set_message("Computing state diff...");
    progress.enable_steady_tick(std::time::Duration::from_millis(100));

    let trace = prism_core::replay::replay_transaction(&args.tx_hash, network).await?;

    progress.finish_and_clear();

    match output_format {
        "json" => println!("{}", serde_json::to_string_pretty(&trace.state_diff)?),
        _ => {
            println!("{}", colored::Colorize::bold("State Diff"));
            println!("{}", renderers::render_state_diff_table(&trace.state_diff));
        }
    }

    Ok(())
}
