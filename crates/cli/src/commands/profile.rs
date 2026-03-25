//! `prism profile` - Resource consumption profile with hotspot analysis.

use clap::Args;
use prism_core::types::config::NetworkConfig;

#[derive(Args)]
pub struct ProfileArgs {
    /// Transaction hash to profile.
    pub tx_hash: String,

    /// Output profile to a file instead of stdout.
    #[arg(long, short)]
    pub output_file: Option<String>,
}

pub async fn run(
    args: ProfileArgs,
    network: &NetworkConfig,
    output_format: &str,
) -> anyhow::Result<()> {
    let progress = indicatif::ProgressBar::new_spinner();
    progress.set_message("Replaying transaction for resource profiling...");
    progress.enable_steady_tick(std::time::Duration::from_millis(100));

    let trace = prism_core::replay::replay_transaction(&args.tx_hash, network).await?;

    progress.finish_and_clear();

    let output = crate::output::format_resource_profile(&trace.resource_profile, output_format)?;

    if let Some(path) = args.output_file {
        std::fs::write(&path, &output)?;
        println!("Profile written to {path}");
    } else {
        println!("{output}");
    }

    Ok(())
}
