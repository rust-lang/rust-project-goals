#!/usr/bin/env python3
"""
Migration script to convert flagship goals from the old format to the new format.

Old format: | Status | Flagship |
New format: | Status | Accepted | + | Flagship | Yes |
"""

import os
import re
import sys
from pathlib import Path

def migrate_file(file_path):
    """Migrate a single goal file from old to new flagship format."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Look for metadata table with Status | Flagship
    # We need to be careful to only match within the metadata table
    lines = content.split('\n')
    
    # Find the metadata table (starts after a line with just "| Metadata" and ends at the first empty line or non-table line)
    in_metadata_table = False
    metadata_start = None
    metadata_end = None
    
    for i, line in enumerate(lines):
        if '| Metadata' in line and '|' in line:
            in_metadata_table = True
            metadata_start = i
            continue
        
        if in_metadata_table:
            # Check if this line is still part of the table
            if line.strip() == '' or not line.strip().startswith('|'):
                metadata_end = i
                break
            
            # Check for Status | Flagship pattern
            if re.match(r'\s*\|\s*Status\s*\|\s*Flagship\s*\|', line):
                # Replace Status | Flagship with Status | Accepted
                lines[i] = re.sub(r'(\|\s*Status\s*\|\s*)Flagship(\s*\|)', r'\1Accepted\2', line)
                
                # Find where to insert the Flagship | Yes row
                # Insert it right after the Status row
                flagship_row = f"| Flagship         | Yes                                |"
                lines.insert(i + 1, flagship_row)
                
                print(f"Migrated {file_path}")
                break
    
    # Write back if changed
    new_content = '\n'.join(lines)
    if new_content != original_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    
    return False

def main():
    """Main migration function."""
    if len(sys.argv) > 1:
        # Specific files provided
        files_to_migrate = [Path(arg) for arg in sys.argv[1:]]
    else:
        # Find all .md files in src/
        src_dir = Path('src')
        if not src_dir.exists():
            print("Error: src/ directory not found. Run this script from the project root.")
            sys.exit(1)
        
        files_to_migrate = list(src_dir.rglob('*.md'))
    
    migrated_count = 0
    
    for file_path in files_to_migrate:
        if file_path.is_file():
            if migrate_file(file_path):
                migrated_count += 1
    
    print(f"Migration complete. {migrated_count} files migrated.")

if __name__ == '__main__':
    main()
