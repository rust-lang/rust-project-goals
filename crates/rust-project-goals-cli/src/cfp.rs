use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use regex::Regex;

/// Module containing pure functions for text processing
pub mod text_processing {
    use regex::Regex;
    
    /// Process template content by replacing placeholders and removing notes
    pub fn process_template_content(content: &str, timeframe: &str, lowercase_timeframe: &str) -> String {
        // Remove note sections (starting with '> **NOTE:**' and continuing until a non-'>' line)
        // But preserve other content starting with '>'
        let lines: Vec<&str> = content.lines().collect();
        let mut filtered_lines = Vec::new();
        let mut in_note_section = false;
        
        for line in lines.iter() {
            if line.trim().starts_with("> **NOTE:**") {
                in_note_section = true;
                continue;
            }
            
            if in_note_section {
                if line.trim().starts_with(">") {
                    continue; // Skip this line as it's part of the note section
                } else {
                    in_note_section = false; // End of note section
                }
            }
            
            filtered_lines.push(*line);
        }
        
        // Join the filtered lines and clean up consecutive empty lines
        let mut cleaned_lines = Vec::new();
        let mut last_was_empty = false;
        
        for line in filtered_lines {
            let is_empty = line.trim().is_empty();
            if !is_empty || !last_was_empty {
                cleaned_lines.push(line);
            }
            last_was_empty = is_empty;
        }
        
        // Join and process placeholders
        cleaned_lines.join("\n")
            .replace("YYYYHN", timeframe)
            .replace("YYYY", &timeframe[0..4])
            .replace("HN", &timeframe[4..])
            // Also add lowercase versions for consistency in file paths
            .replace("yyyyhn", lowercase_timeframe)
            .replace("yyyy", &lowercase_timeframe[0..4])
            .replace("hn", &lowercase_timeframe[4..])
    }
    
    /// Process SUMMARY.md content to add or update a timeframe section
    pub fn process_summary_content(content: &str, timeframe: &str, lowercase_timeframe: &str) -> String {
        let mut new_content = content.to_string();
        
        // Create the new section content with capitalized H
        let capitalized_timeframe = format!("{}H{}", &timeframe[0..4], &timeframe[5..]);
        let new_section_content = format!(
            "# ⏳ {} goal process\n\n\
             - [Overview](./{}/README.md)\n\
             - [Proposed goals](./{}/goals.md)\n\
             - [Goals not accepted](./{}/not_accepted.md)\n",
            capitalized_timeframe, lowercase_timeframe, lowercase_timeframe, lowercase_timeframe
        );
        
        // Check if the timeframe is already in the SUMMARY.md
        let section_header = format!("# ⏳ {} goal process", capitalized_timeframe);
        
        if content.contains(&section_header) {
            // The section exists, but it might have placeholder content
            // Find the section header and its content, but be careful not to include the next section header
            let section_header_pattern = format!(r"# ⏳ {} goal process", regex::escape(&capitalized_timeframe));
            let re = Regex::new(&section_header_pattern).unwrap();
            
            if let Some(section_match) = re.find(&content) {
                // Find the end of the section header line
                if let Some(header_end) = content[section_match.end()..].find('\n') {
                    let content_start = section_match.end() + header_end + 1;
                    
                    // Find the start of the next section header (if any)
                    let next_section_start = content[content_start..].find("\n# ")
                        .map(|pos| content_start + pos)
                        .unwrap_or(content.len());
                    
                    // Extract the section content
                    let section_content = &content[content_start..next_section_start];
                    
                    // Check if it contains placeholder content like "- [Not yet started]()"
                    if section_content.contains("[Not yet started]") || section_content.trim().is_empty() {
                        // Format the replacement content (just the links, not the section header)
                        let replacement_content = format!(
                            "\n- [Overview](./{}/README.md)\n\
                             - [Proposed goals](./{}/goals.md)\n\
                             - [Goals not accepted](./{}/not_accepted.md)\n",
                            lowercase_timeframe, lowercase_timeframe, lowercase_timeframe
                        );
                        
                        // Replace just the section content, preserving the section header and any following sections
                        new_content.replace_range(content_start..next_section_start, &replacement_content);
                        return new_content;
                    } else {
                        // Section already has non-placeholder content, don't modify it
                        return content.to_string();
                    }
                }
            }
        }
        
        // If we get here, the section doesn't exist, so we need to add it
        let new_section = format!("\n{}", new_section_content);
        
        // Find a good place to insert the new section
        // Look for the last timeframe section or insert at the beginning
        // Match both lowercase and uppercase H
        let re = Regex::new(r"# ⏳ \d{4}[hH][12] goal process").unwrap();
        
        if let Some(last_match) = re.find_iter(&content).last() {
            // Find the end of this section (next section or end of file)
            if let Some(next_section_pos) = content[last_match.end()..].find("\n# ") {
                let insert_pos = last_match.end() + next_section_pos;
                new_content.insert_str(insert_pos, &new_section);
            } else {
                // No next section, append to the end
                new_content.push_str(&new_section);
            }
        } else {
            // No existing timeframe sections, insert at the beginning
            new_content = new_section + &content;
        }
        
        new_content
    }
    
    /// Process README.md content to add or update a timeframe section
    pub fn process_readme_content(content: &str, timeframe: &str, lowercase_timeframe: &str) -> String {
        let mut new_content = content.to_string();
        
        // Extract year and half from timeframe
        let _year = &timeframe[0..4];
        let half = &timeframe[4..].to_lowercase();
        
        // Determine the months based on the half
        let (start_month, end_month) = if half == "h1" {
            ("January", "June")
        } else {
            ("July", "December")
        };
        
        // Create the new section to add with capitalized H
        let capitalized_timeframe = format!("{}H{}", &timeframe[0..4], &timeframe[5..]);
        let new_section = format!(
            "\n## Next goal period ({})\n\n\
             The next goal period will be {}, running from the start of {} to the end of {}. \
             We are currently in the process of assembling goals. \
             [Click here](./{}/goals.md) to see the current list. \
             If you'd like to propose a goal, [instructions can be found here](./how_to/propose_a_goal.md).\n",
            capitalized_timeframe, capitalized_timeframe, start_month, end_month, lowercase_timeframe
        );
        
        // First check for an existing entry for this specific timeframe
        let this_period_pattern = Regex::new(&format!(r"## Next goal period(?:\s*\({}\))?\s*\n", regex::escape(&capitalized_timeframe))).unwrap();
        
        // If not found, look for any "Next goal period" section
        let next_period_pattern = Regex::new(r"## Next goal period(?:\s*\([^\)]*\))?\s*\n").unwrap();
        
        // Also look for "Current goal period" to place after it if no "Next goal period" exists
        let current_period_pattern = Regex::new(r"## Current goal period(?:\s*\([^\)]*\))?\s*\n").unwrap();
        
        // First try to find and replace an existing entry for this specific timeframe
        if let Some(this_period_match) = this_period_pattern.find(&content) {
            // Found an existing section for this specific timeframe
            // Find the end of this section (next section or end of file)
            if let Some(next_section_pos) = content[this_period_match.end()..].find("\n## ") {
                let end_pos = this_period_match.start() + next_section_pos + this_period_match.end() - this_period_match.start();
                new_content.replace_range(this_period_match.start()..end_pos, &new_section);
            } else {
                // No next section, replace until the end
                new_content.replace_range(this_period_match.start().., &new_section);
            }
        } else if let Some(next_period_match) = next_period_pattern.find(&content) {
            // Found an existing "Next goal period" section
            // Find the end of this section (next section or end of file)
            if let Some(next_section_pos) = content[next_period_match.end()..].find("\n## ") {
                let end_pos = next_period_match.start() + next_section_pos + next_period_match.end() - next_period_match.start();
                new_content.replace_range(next_period_match.start()..end_pos, &new_section);
            } else {
                // No next section, replace until the end
                new_content.replace_range(next_period_match.start().., &new_section);
            }
        } else {
            // No existing "Next goal period" section, try to add after "Current goal period" section
            if let Some(current_period_match) = current_period_pattern.find(&content) {
                // Find the end of the current period section
                if let Some(next_section_pos) = content[current_period_match.end()..].find("\n## ") {
                    let insert_pos = current_period_match.end() + next_section_pos;
                    new_content.insert_str(insert_pos, &new_section);
                } else {
                    // No next section after current period, append to the end
                    new_content.push_str(&new_section);
                }
            } else {
                // No "Current goal period" section either, add after the introduction
                if let Some(pos) = content.find("\n## ") {
                    new_content.insert_str(pos, &new_section);
                } else {
                    // Fallback: append to the end
                    new_content.push_str(&new_section);
                }
            }
        }
        
        new_content
    }
}

/// Creates the directory structure and files for a new Call For Proposals (CFP) period.
pub fn create_cfp(timeframe: &str, force: bool, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("Dry run mode - no changes will be made");
    }
    // Validate the timeframe format
    validate_timeframe(timeframe)?;
    
    // Ensure the timeframe is lowercase for directory and file paths
    let lowercase_timeframe = timeframe.to_lowercase();
    
    // Create the directory for the new timeframe (always lowercase)
    let dir_path = PathBuf::from("src").join(&lowercase_timeframe);
    if dir_path.exists() {
        if !force && !dry_run {
            println!("Directory {} already exists. Continuing will overwrite files.", dir_path.display());
            println!("Do you want to continue? [y/N]");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                return Ok(());
            }
        } else if force {
            println!("Directory {} already exists. Force flag set, continuing without confirmation.", dir_path.display());
        } else {
            // dry_run mode
            println!("Would create/overwrite directory: {}", dir_path.display());
        }
    } else if !dry_run {
        fs::create_dir_all(&dir_path).with_context(|| format!("Failed to create directory {}", dir_path.display()))?;
        println!("Created directory: {}", dir_path.display());
    } else {
        println!("Would create directory: {}", dir_path.display());
    }
    
    // Copy and process template files
    copy_and_process_template("src/admin/samples/rfc.md", &dir_path.join("README.md"), timeframe, &lowercase_timeframe, dry_run)?;
    copy_and_process_template("src/admin/samples/goals.md", &dir_path.join("goals.md"), timeframe, &lowercase_timeframe, dry_run)?;
    copy_and_process_template("src/admin/samples/not_accepted.md", &dir_path.join("not_accepted.md"), timeframe, &lowercase_timeframe, dry_run)?;
    
    // Update SUMMARY.md
    update_summary_md(timeframe, &lowercase_timeframe, dry_run)?;
    
    // Update main README.md with the new timeframe section
    update_main_readme(timeframe, &lowercase_timeframe, dry_run)?;
    
    println!("\nCFP setup for {} completed successfully!", timeframe);
    println!("\nNext steps:");
    println!("1. Review and customize the generated files in src/{}/", lowercase_timeframe);
    println!("2. Prepare a Call For Proposals blog post based on the sample in src/admin/samples/cfp.md");
    println!("3. Run 'mdbook build' to generate the book with the new content");
    
    Ok(())
}

/// Validates that the timeframe is in the correct format (e.g., "2025h1" or "2025H1")
fn validate_timeframe(timeframe: &str) -> Result<()> {
    let re = Regex::new(r"^\d{4}[hH][12]$").unwrap();
    if !re.is_match(timeframe) {
        anyhow::bail!("Invalid timeframe format. Expected format: YYYYhN or YYYYHN (e.g., 2025h1, 2025H1, 2025h2, or 2025H2)");
    }
    Ok(())
}

/// Copies a template file to the destination and replaces placeholders
fn copy_and_process_template(template_path: &str, dest_path: &Path, timeframe: &str, lowercase_timeframe: &str, dry_run: bool) -> Result<()> {
    // Read the template file
    let template_content = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read template file: {}", template_path))?;
    
    // Use the pure function to process the content
    let processed_content = text_processing::process_template_content(&template_content, timeframe, lowercase_timeframe);
    
    // Write to destination file
    if !dry_run {
        File::create(dest_path)
            .with_context(|| format!("Failed to create file: {}", dest_path.display()))?
            .write_all(processed_content.as_bytes())
            .with_context(|| format!("Failed to write to file: {}", dest_path.display()))?;
        
        println!("Created file: {}", dest_path.display());
    } else {
        println!("Would create file: {}", dest_path.display());
    }
    Ok(())
}

/// Updates the SUMMARY.md file to include the new timeframe section
fn update_summary_md(timeframe: &str, lowercase_timeframe: &str, dry_run: bool) -> Result<()> {
    let summary_path = "src/SUMMARY.md";
    let content = fs::read_to_string(summary_path)
        .with_context(|| format!("Failed to read SUMMARY.md"))?;
    
    // Use the pure function to process the content
    let new_content = text_processing::process_summary_content(&content, timeframe, lowercase_timeframe);
    
    // Check if the content was modified
    if new_content == content {
        if !dry_run {
            println!("SUMMARY.md already contains a non-placeholder entry for {}. Skipping update.", timeframe);
        } else {
            println!("Would skip updating SUMMARY.md (already contains a non-placeholder entry for {})", timeframe);
        }
        return Ok(());
    }
    
    // Write the updated content back to SUMMARY.md
    if !dry_run {
        fs::write(summary_path, new_content)
            .with_context(|| format!("Failed to write to SUMMARY.md"))?;
        
        println!("Updated SUMMARY.md with {} section", timeframe);
    } else {
        println!("Would update SUMMARY.md with {} section", timeframe);
    }
    
    Ok(())
}

/// Updates the src/README.md with information about the new timeframe
fn update_main_readme(timeframe: &str, lowercase_timeframe: &str, dry_run: bool) -> Result<()> {
    let readme_path = "src/README.md";
    let content = fs::read_to_string(readme_path)
        .with_context(|| format!("Failed to read README.md"))?;
    
    // Use the pure function to process the content
    let new_content = text_processing::process_readme_content(&content, timeframe, lowercase_timeframe);
    
    // Check if the content was modified
    if new_content == content {
        if !dry_run {
            println!("README.md already contains up-to-date information for {}. Skipping update.", timeframe);
        } else {
            println!("Would skip updating README.md (already contains up-to-date information for {})", timeframe);
        }
        return Ok(());
    }
    
    // Determine what kind of update was made for better logging
    let capitalized_timeframe = format!("{}H{}", &timeframe[0..4], &timeframe[5..]);
    let specific_timeframe_pattern = format!(r"## Next goal period(?:\s*\({}\))", regex::escape(&capitalized_timeframe));
    let specific_re = Regex::new(&specific_timeframe_pattern).unwrap();
    
    if specific_re.is_match(&content) && specific_re.is_match(&new_content) {
        if !dry_run {
            println!("Updated existing 'Next goal period ({})' section in src/README.md", capitalized_timeframe);
        } else {
            println!("Would update existing 'Next goal period ({})' section in src/README.md", capitalized_timeframe);
        }
    } else if Regex::new(r"## Next goal period(?:\s*\([^\)]*\))").unwrap().is_match(&content) {
        if !dry_run {
            println!("Updated existing 'Next goal period' section in src/README.md");
        } else {
            println!("Would update existing 'Next goal period' section in src/README.md");
        }
    } else if Regex::new(r"## Current goal period(?:\s*\([^\)]*\))").unwrap().is_match(&content) {
        if !dry_run {
            println!("Added new 'Next goal period' section after 'Current goal period' in src/README.md");
        } else {
            println!("Would add new 'Next goal period' section after 'Current goal period' in src/README.md");
        }
    } else {
        if !dry_run {
            println!("Added new 'Next goal period' section to src/README.md");
        } else {
            println!("Would add new 'Next goal period' section to src/README.md");
        }
    }
    
    // Write the updated content back to README.md
    if !dry_run {
        fs::write(readme_path, new_content)
            .with_context(|| format!("Failed to write to src/README.md"))?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::text_processing::*;

    #[test]
    fn test_process_template_content() {
        // Test basic placeholder replacement
        let content = "# YYYYHN Goals\n\nThis is for YYYY and HN.";
        let result = process_template_content(content, "2026H1", "2026h1");
        assert_eq!(result, "# 2026H1 Goals\n\nThis is for 2026 and H1.");
        
        // Test note removal
        let content_with_notes = "# YYYYHN Goals\n\n> **NOTE:** This is a note that should be removed.\n> More note content.\n\nThis should stay.";
        let result = process_template_content(content_with_notes, "2026H1", "2026h1");
        assert_eq!(result, "# 2026H1 Goals\n\nThis should stay.");
        
        // Test that other blockquotes are preserved
        let content_with_blockquote = "# YYYYHN Goals\n\n> This is a regular blockquote that should stay.\n\n> **NOTE:** This is a note that should be removed.\n> More note content.\n\nThis should stay.";
        let result = process_template_content(content_with_blockquote, "2026H1", "2026h1");
        assert_eq!(result, "# 2026H1 Goals\n\n> This is a regular blockquote that should stay.\n\nThis should stay.");
    }
    
    #[test]
    fn test_process_summary_content_no_existing_section() {
        // Test adding a new section when no timeframe sections exist
        let content = "# Summary\n\n[Introduction](./README.md)\n";
        let result = process_summary_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("# ⏳ 2026H1 goal process"));
        assert!(result.contains("- [Overview](./2026h1/README.md)"));
        assert!(result.contains("- [Proposed goals](./2026h1/goals.md)"));
        assert!(result.contains("- [Goals not accepted](./2026h1/not_accepted.md)"));
    }
    
    #[test]
    fn test_process_summary_content_with_placeholder() {
        // Test updating a section that has placeholder content
        let content = "# Summary\n\n[Introduction](./README.md)\n\n# ⏳ 2026H1 goal process\n\n- [Not yet started]()\n";
        let result = process_summary_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("# ⏳ 2026H1 goal process"));
        assert!(result.contains("- [Overview](./2026h1/README.md)"));
        assert!(!result.contains("- [Not yet started]()"));
    }
    
    #[test]
    fn test_process_summary_content_with_existing_content() {
        // Test that existing non-placeholder content is not modified
        let content = "# Summary\n\n[Introduction](./README.md)\n\n# ⏳ 2026H1 goal process\n\n- [Already populated](./2026h1/README.md)\n";
        let result = process_summary_content(content, "2026h1", "2026h1");
        
        // Should not change existing non-placeholder content
        assert_eq!(result, content);
    }
    
    #[test]
    fn test_process_summary_content_with_other_timeframes() {
        // Test adding a new section when other timeframe sections exist
        let content = "# Summary\n\n[Introduction](./README.md)\n\n# ⏳ 2025H1 goal process\n\n- [Overview](./2025h1/README.md)\n";
        let result = process_summary_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("# ⏳ 2025H1 goal process"));
        assert!(result.contains("# ⏳ 2026H1 goal process"));
        assert!(result.contains("- [Overview](./2026h1/README.md)"));
    }
    
    #[test]
    fn test_process_readme_content_no_existing_section() {
        // Test adding a new section when no next goal period section exists
        let content = "# Project goals\n\n## Current goal period (2025H1)\n\nThe 2025H1 goal period runs from Jan 1 to Jun 30.";
        let result = process_readme_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("## Next goal period (2026H1)"));
        assert!(result.contains("running from January 1 to June 30"));
        assert!(result.contains("[Click here](./2026h1/goals.md)"));
    }
    
    #[test]
    fn test_process_readme_content_with_existing_section() {
        // Test updating an existing section for the same timeframe
        let content = "# Project goals\n\n## Current goal period (2025H1)\n\nThe 2025H1 goal period runs from Jan 1 to Jun 30.\n\n## Next goal period (2026H1)\n\nOld content.";
        let result = process_readme_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("## Next goal period (2026H1)"));
        assert!(result.contains("running from January 1 to June 30"));
        assert!(!result.contains("Old content."));
    }

    #[test]
    fn test_process_readme_content_with_existing_section_and_extra() {
        // Test updating an existing section while preserving unrelated sections
        let content = "# Project goals\n\n## Current goal period (2025H1)\n\nThe 2025H1 goal period runs from Jan 1 to Jun 30.\n\n## Next goal period (2026H1)\n\nOld content.\n\n## Extra section\nsome content";
        let result = process_readme_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("## Next goal period (2026H1)"));
        assert!(result.contains("running from January 1 to June 30"));
        assert!(!result.contains("Old content."));
        assert!(result.contains("## Extra section\nsome content"));
    }
    
    #[test]
    fn test_process_readme_content_with_different_timeframe() {
        // Test replacing an existing next goal period section with a different timeframe
        let content = "# Project goals\n\n## Current goal period (2025H1)\n\nThe 2025H1 goal period runs from Jan 1 to Jun 30.\n\n## Next goal period (2025H2)\n\nOld content.";
        let result = process_readme_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("## Next goal period (2026H1)"));
        assert!(!result.contains("## Next goal period (2025H2)"));
        assert!(result.contains("running from January 1 to June 30"));
    }
    
    #[test]
    fn test_process_readme_content_second_half() {
        // Test that the correct months are used for the second half of the year
        let content = "# Project goals\n\n## Current goal period (2025H2)\n\nThe 2025H2 goal period runs from Jul 1 to Dec 31.";
        let result = process_readme_content(content, "2026h2", "2026h2");
        
        assert!(result.contains("## Next goal period (2026H2)"));
        assert!(result.contains("running from the start of July to the end of December"));
    }
    
    #[test]
    fn test_process_readme_content_no_current_period() {
        // Test adding a section when there's no current goal period section
        let content = "# Project goals\n\n## About the process\n\nSome content.";
        let result = process_readme_content(content, "2026h1", "2026h1");
        
        assert!(result.contains("## Next goal period (2026H1)"));
        assert!(result.contains("running from the start of January to the end of June"));
    }
}
