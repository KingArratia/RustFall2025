use std::env;
use std::process;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry}; 

// Using the raw identifier r#final to escape the reserved keyword
use r#final::{analyze_file, FileAnalysis, ProcessingError};

const DEFAULT_PATH: &str = ".";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_process = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_PATH);

    println!("--- Parallel File Processor Initialized ---");
    println!("Targeting directory: '{}'", path_to_process);
    
    let num_threads = rayon::current_num_threads();
    println!("Using Rayon thread pool with {} worker threads.", num_threads);

    // --- File Discovery ---
    let entries: Vec<DirEntry> = WalkDir::new(path_to_process)
        .into_iter()
        .filter_map(|e| {
            match e {
                Ok(entry) => {
                    let path = entry.path();
                    
                    // FIX: Filtering Logic to skip 'target' directory (TA recommendation)
                    if path.components().any(|c| c.as_os_str() == "target") {
                        return None;
                    }
                    
                    // Only keep files
                    if entry.file_type().is_file() {
                        Some(entry)
                    } else {
                        None // Skip directories, symlinks, etc.
                    }
                }
                Err(e) => {
                    eprintln!("Error during directory traversal: {}", e);
                    None
                }
            }
        })
        .collect();

    let total_files = entries.len();
    if total_files == 0 {
        println!("No files found to process in the specified directory.");
        process::exit(0);
    }
    println!("Found {} files. Starting parallel processing...", total_files);
    
    // --- Parallel Processing ---
    // Rayon uses the thread pool implicitly here. 
    let results: Vec<FileAnalysis> = entries
        .into_par_iter()
        .map(analyze_file)
        .collect();

    // --- Output ---
    
    let successful_count = results.iter().filter(|r| r.errors.is_empty()).count();
    let failed_count = total_files - successful_count;
    
    println!("\n--- Processing Complete ---");
    println!("Total Files Processed: {}", total_files);
    println!("Successful Analyses: {}", successful_count);
    println!("Files with Errors: {}", failed_count);
    
    println!("\n--- Detailed Results ---");
    
    for analysis in results {
        let status = if analysis.errors.is_empty() {
            "SUCCESS"
        } else {
            "ERROR"
        };

        println!("\nFile: **{}** ({})", analysis.filename, status);
        
        if status == "SUCCESS" {
            println!("  Time: {:.2?}", analysis.processing_time);
            println!("  Stats: W: {}, L: {}, S: {} bytes", 
                     analysis.stats.word_count, 
                     analysis.stats.line_count, 
                     analysis.stats.size_bytes);
        } else {
            println!("  Errors Encountered:");
            for error in analysis.errors {
                match error {
                    ProcessingError::Io(e) => println!("    -> IO Error: {}", e),
                    ProcessingError::Cancellation => println!("    -> Cancellation signaled."),
                }
            }
        }
    }
}