//! Output renderers for CLI reports.
//!
//! This module contains specialized renderers for different output formats.

use prism_core::types::report::{DiagnosticReport, SuggestedFix};

/// Renders a bulleted list of suggested fixes from the diagnostic report.
///
/// Each fix is displayed with a distinctive icon indicating its characteristics:
/// - 🔧 for standard fixes
/// - ⚠️ for fixes that require a contract upgrade
/// - 📋 for fixes with code examples
pub fn render_fix_list(report: &DiagnosticReport) -> String {
    if report.suggested_fixes.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str("Actionable Fixes:\n");

    for (index, fix) in report.suggested_fixes.iter().enumerate() {
        let icon = get_fix_icon(fix);
        let difficulty_badge = get_difficulty_badge(&fix.difficulty);
        
        output.push_str(&format!("  {} {}{}\n", icon, fix.description, difficulty_badge));
        
        if fix.requires_upgrade {
            output.push_str("    ⚡ May require contract upgrade\n");
        }
        
        if let Some(example) = &fix.example {
            output.push_str(&format!("    📄 Example: {}\n", example));
        }
        
        // Add a blank line between fixes except for the last one
        if index < report.suggested_fixes.len() - 1 {
            output.push('\n');
        }
    }

    output
}

/// Returns the appropriate icon for a suggested fix based on its characteristics.
fn get_fix_icon(fix: &SuggestedFix) -> &'static str {
    if fix.requires_upgrade {
        "🔒"
    } else if fix.example.is_some() {
        "📋"
    } else {
        "🔧"
    }
}

/// Returns a badge indicating the difficulty level of the fix.
fn get_difficulty_badge(difficulty: &str) -> String {
    match difficulty.to_lowercase().as_str() {
        "easy" => " [easy]".to_string(),
        "medium" => " [medium]".to_string(),
        "hard" => " [hard]".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::report::Severity;

    fn create_test_report() -> DiagnosticReport {
        DiagnosticReport {
            error_category: "Budget".to_string(),
            error_code: 1,
            error_name: "cpu_limit_exceeded".to_string(),
            summary: "CPU usage exceeded limit".to_string(),
            detailed_explanation: "The contract used more CPU than allowed.".to_string(),
            severity: Severity::Error,
            root_causes: vec![],
            suggested_fixes: vec![
                SuggestedFix {
                    description: "Reduce the number of loop iterations".to_string(),
                    difficulty: "easy".to_string(),
                    requires_upgrade: false,
                    example: Some("Use for_each instead of iterate".to_string()),
                },
                SuggestedFix {
                    description: "Optimize your contract logic".to_string(),
                    difficulty: "medium".to_string(),
                    requires_upgrade: false,
                    example: None,
                },
                SuggestedFix {
                    description: "Upgrade to a newer contract version".to_string(),
                    difficulty: "hard".to_string(),
                    requires_upgrade: true,
                    example: None,
                },
            ],
            contract_error: None,
            transaction_context: None,
            related_errors: vec![],
        }
    }

    #[test]
    fn test_render_fix_list_with_fixes() {
        let report = create_test_report();
        let output = render_fix_list(&report);
        
        assert!(output.contains("Actionable Fixes:"));
        assert!(output.contains("🔧"));
        assert!(output.contains("📋"));
        assert!(output.contains("🔒"));
        assert!(output.contains("[easy]"));
        assert!(output.contains("[medium]"));
        assert!(output.contains("[hard]"));
        assert!(output.contains("May require contract upgrade"));
    }

    #[test]
    fn test_render_fix_list_empty() {
        let mut report = create_test_report();
        report.suggested_fixes = vec![];
        let output = render_fix_list(&report);
        
        assert!(output.is_empty());
    }

    #[test]
    fn test_get_fix_icon() {
        let fix_with_example = SuggestedFix {
            description: "Test".to_string(),
            difficulty: "easy".to_string(),
            requires_upgrade: false,
            example: Some("code".to_string()),
        };
        assert_eq!(get_fix_icon(&fix_with_example), "📋");

        let fix_requires_upgrade = SuggestedFix {
            description: "Test".to_string(),
            difficulty: "easy".to_string(),
            requires_upgrade: true,
            example: None,
        };
        assert_eq!(get_fix_icon(&fix_requires_upgrade), "🔒");

        let fix_standard = SuggestedFix {
            description: "Test".to_string(),
            difficulty: "easy".to_string(),
            requires_upgrade: false,
            example: None,
        };
        assert_eq!(get_fix_icon(&fix_standard), "🔧");
    }

    #[test]
    fn test_get_difficulty_badge() {
        assert_eq!(get_difficulty_badge("easy"), " [easy]");
        assert_eq!(get_difficulty_badge("medium"), " [medium]");
        assert_eq!(get_difficulty_badge("hard"), " [hard]");
        assert_eq!(get_difficulty_badge("unknown"), "");
    }
}
