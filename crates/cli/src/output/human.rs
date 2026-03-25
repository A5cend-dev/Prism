//! Human-readable colored terminal output formatter.

use prism_core::types::report::DiagnosticReport;

use crate::output::renderers::render_fix_list;

/// Print a diagnostic report in human-readable colored format.
pub fn print_report(report: &DiagnosticReport) -> anyhow::Result<()> {
    // TODO: Implement rich colored terminal output
    println!(
        "Error: {} ({}:{})",
        report.error_name, report.error_category, report.error_code
    );
    println!("Summary: {}", report.summary);
    
    // Render and print the actionable fixes section
    let fix_list = render_fix_list(report);
    if !fix_list.is_empty() {
        println!();
        print!("{}", fix_list);
    }
    
    Ok(())
}
