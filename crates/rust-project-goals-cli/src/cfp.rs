use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use regex::Regex;

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
    let mut template_content = String::new();
    File::open(template_path)
        .with_context(|| format!("Failed to open template file: {}", template_path))?
        .read_to_string(&mut template_content)
        .with_context(|| format!("Failed to read template file: {}", template_path))?;
    
    // Remove note sections (starting with '> **NOTE:**' and continuing until a non-'>' line)
    // But preserve other content starting with '>'
    let lines: Vec<&str> = template_content.lines().collect();
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
    
    // Join the filtered lines back together
    let template_content = filtered_lines.join("\n");
    
    // Replace placeholders
    let processed_content = template_content
        .replace("YYYYHN", timeframe)
        .replace("YYYY", &timeframe[0..4])
        .replace("HN", &timeframe[4..])
        // Also add lowercase versions for consistency in file paths
        .replace("yyyyhn", lowercase_timeframe)
        .replace("yyyy", &lowercase_timeframe[0..4])
        .replace("hn", &lowercase_timeframe[4..]);
    
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
    let mut content = String::new();
    File::open(summary_path)
        .with_context(|| format!("Failed to open SUMMARY.md"))?
        .read_to_string(&mut content)
        .with_context(|| format!("Failed to read SUMMARY.md"))?;
    
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
        
        if let Some(section_start) = content.find(&section_header_pattern) {
            // Find the end of the section header line
            if let Some(header_end) = content[section_start..].find('\n') {
                let content_start = section_start + header_end + 1;
                
                // Find the start of the next section header (if any)
                let next_section_start = content[content_start..].find("\n# ")
                    .map(|pos| content_start + pos)
                    .unwrap_or(content.len());
                
                // Extract the section content
                let section_content = &content[content_start..next_section_start];
                
                // Check if it contains placeholder content like "- [Not yet started]()"
                if section_content.contains("[Not yet started]") || section_content.trim().is_empty() {
                    // Create the new content
                    let mut new_content = content.clone();
                    
                    // Format the replacement content (just the links, not the section header)
                    let replacement_content = format!(
                        "\n- [Overview](./{}/README.md)\n\
                         - [Proposed goals](./{}/goals.md)\n\
                         - [Goals not accepted](./{}/not_accepted.md)\n",
                        lowercase_timeframe, lowercase_timeframe, lowercase_timeframe
                    );
                    
                    // Replace just the section content, preserving the section header and any following sections
                    new_content.replace_range(content_start..next_section_start, &replacement_content);
                    
                    // Write the updated content back to SUMMARY.md
                    if !dry_run {
                        File::create(summary_path)
                            .with_context(|| format!("Failed to open SUMMARY.md for writing"))?
                            .write_all(new_content.as_bytes())
                            .with_context(|| format!("Failed to write to SUMMARY.md"))?;
                        
                        println!("Updated existing placeholder section for {} in SUMMARY.md", timeframe);
                    } else {
                        println!("Would update existing placeholder section for {} in SUMMARY.md", timeframe);
                    }
                    return Ok(());
                } else {
                    if !dry_run {
                        println!("SUMMARY.md already contains a non-placeholder entry for {}. Skipping update.", timeframe);
                    } else {
                        println!("Would skip updating SUMMARY.md (already contains a non-placeholder entry for {})", timeframe);
                    }
                    return Ok(());
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
    let mut new_content = content.clone();
    
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
    
    // Write the updated content back to SUMMARY.md
    if !dry_run {
        File::create(summary_path)
            .with_context(|| format!("Failed to open SUMMARY.md for writing"))?
            .write_all(new_content.as_bytes())
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
    let mut content = String::new();
    File::open(readme_path)
        .with_context(|| format!("Failed to open README.md"))?
        .read_to_string(&mut content)
        .with_context(|| format!("Failed to read README.md"))?;
    
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
         The next goal period will be {}, running from {} 1 to {} 30. \
         We are currently in the process of assembling goals. \
         [Click here](./{}/goals.md) to see the current list. \
         If you'd like to propose a goal, [instructions can be found here](./how_to/propose_a_goal.md).\n",
        capitalized_timeframe, capitalized_timeframe, start_month, end_month, lowercase_timeframe
    );
    
    // Find a good place to insert the new section
    // Look for an existing "Next goal period" section or add after the current goal period section
    let mut new_content = content.clone();
    
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
        if !dry_run {
            println!("Updated existing 'Next goal period ({})' section in src/README.md", capitalized_timeframe);
        } else {
            println!("Would update existing 'Next goal period ({})' section in src/README.md", capitalized_timeframe);
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
        if !dry_run {
            println!("Updated existing 'Next goal period' section in src/README.md");
        } else {
            println!("Would update existing 'Next goal period' section in src/README.md");
        }
    } else {
        // No existing "Next goal period" section, try to add after "Current goal period" section
        if let Some(current_period_match) = current_period_pattern.find(&content) {
            // Find the end of the current period section
            if let Some(next_section_pos) = content[current_period_match.end()..].find("\n## ") {
                let insert_pos = current_period_match.end() + next_section_pos;
                new_content.insert_str(insert_pos, &new_section);
                if !dry_run {
                    println!("Added new 'Next goal period' section after 'Current goal period' in src/README.md");
                } else {
                    println!("Would add new 'Next goal period' section after 'Current goal period' in src/README.md");
                }
            } else {
                // No next section after current period, append to the end
                new_content.push_str(&new_section);
                if !dry_run {
                    println!("Appended 'Next goal period' section to src/README.md");
                } else {
                    println!("Would append 'Next goal period' section to src/README.md");
                }
            }
        } else {
            // No "Current goal period" section either, add after the introduction
            if let Some(pos) = content.find("\n## ") {
                new_content.insert_str(pos, &new_section);
                if !dry_run {
                    println!("Added new 'Next goal period' section to src/README.md");
                } else {
                    println!("Would add new 'Next goal period' section to src/README.md");
                }
            } else {
                // Fallback: append to the end
                new_content.push_str(&new_section);
                if !dry_run {
                    println!("Appended 'Next goal period' section to src/README.md");
                } else {
                    println!("Would append 'Next goal period' section to src/README.md");
                }
            }
        }
    }
    
    // Write the updated content back to README.md
    if !dry_run {
        File::create(readme_path)
            .with_context(|| format!("Failed to open src/README.md for writing"))?
            .write_all(new_content.as_bytes())
            .with_context(|| format!("Failed to write to src/README.md"))?;
    }
    
    Ok(())
}
