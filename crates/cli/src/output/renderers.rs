//! Table renderers for CLI output.

use colored::Colorize;
use tabled::{ settings::Style, Table, Tabled };

use prism_core::types::trace::{ LedgerEntryDiff, StateDiff };

/// Row representation for StateDiff table
#[derive(Tabled)]
struct StateDiffRow {
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Before")]
    before: String,
    #[tabled(rename = "After")]
    after: String,
}

/// Render a StateDiff as a formatted table with highlighting
pub fn render_state_diff_table(state_diff: &StateDiff) -> String {
    let rows: Vec<StateDiffRow> = state_diff.entries
        .iter()
        .map(|entry| {
            let before = match &entry.before {
                Some(value) => {
                    if
                        matches!(
                            entry.change_type,
                            prism_core::types::trace::DiffChangeType::Deleted
                        )
                    {
                        value.red().to_string()
                    } else {
                        value.clone()
                    }
                }
                None => "-".to_string(),
            };

            let after = match &entry.after {
                Some(value) => {
                    if
                        matches!(
                            entry.change_type,
                            prism_core::types::trace::DiffChangeType::Created
                        )
                    {
                        value.green().to_string()
                    } else if
                        matches!(
                            entry.change_type,
                            prism_core::types::trace::DiffChangeType::Updated
                        )
                    {
                        value.green().to_string()
                    } else {
                        value.clone()
                    }
                }
                None => "-".to_string(),
            };

            let key = match entry.change_type {
                prism_core::types::trace::DiffChangeType::Created =>
                    format!("+ {}", entry.key).green().to_string(),
                prism_core::types::trace::DiffChangeType::Deleted =>
                    format!("- {}", entry.key).red().to_string(),
                prism_core::types::trace::DiffChangeType::Updated =>
                    format!("~ {}", entry.key).yellow().to_string(),
                prism_core::types::trace::DiffChangeType::Unchanged => entry.key.clone(),
            };

            StateDiffRow { key, before, after }
        })
        .collect();

    Table::new(&rows).with(Style::modern()).to_string()
}

/// Render a single LedgerEntryDiff for detailed view
pub fn render_ledger_entry_diff(entry: &LedgerEntryDiff) -> String {
    let change_symbol = match entry.change_type {
        prism_core::types::trace::DiffChangeType::Created => "+".green().to_string(),
        prism_core::types::trace::DiffChangeType::Deleted => "-".red().to_string(),
        prism_core::types::trace::DiffChangeType::Updated => "~".yellow().to_string(),
        prism_core::types::trace::DiffChangeType::Unchanged => " ".dimmed().to_string(),
    };

    let before_value = entry.before.as_deref().unwrap_or("-");
    let after_value = entry.after.as_deref().unwrap_or("-");

    format!(
        "{} {}\n  Before: {}\n  After:  {}",
        change_symbol,
        entry.key,
        before_value.red(),
        after_value.green()
    )
}
