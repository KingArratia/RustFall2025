use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::time::Duration;
use walkdir::DirEntry;

// --- Required Output Structs ---

#[derive(Debug, Default)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub enum ProcessingError {
    Io(io::Error),
    Cancellation, 
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

// --- Core Analyzer Function ---

pub fn analyze_file(entry: DirEntry) -> FileAnalysis {
    use std::time::Instant;
    
    let start_time = Instant::now();
    let path = entry.path();
    let filename = path.display().to_string();
    let mut stats = FileStats::default();
    let mut errors = Vec::new();
    
    match fs::metadata(path) {
        Ok(meta) => {
            stats.size_bytes = meta.len();
        },
        Err(e) => {
            errors.push(ProcessingError::Io(e));
            return FileAnalysis {
                filename,
                stats,
                errors,
                processing_time: start_time.elapsed(),
            };
        }
    }

    match fs::File::open(path).and_then(|mut f| {
        let mut content = String::new();
        f.read_to_string(&mut content).map(|_| content)
    }) {
        Ok(content) => {
            stats.line_count = content.lines().count();
            
            let mut word_count = 0;
            for word in content.split_whitespace() {
                word_count += 1;
                for ch in word.chars() {
                    *stats.char_frequencies.entry(ch).or_insert(0) += 1;
                }
            }
            stats.word_count = word_count;
        }
        Err(e) => {
            errors.push(ProcessingError::Io(e));
        }
    }

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}