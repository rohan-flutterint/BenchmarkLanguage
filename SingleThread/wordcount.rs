use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let filename = "../largefile.txt";
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::with_capacity(16 * 1024 * 1024, file); // 16 MB buffer
    
    let mut word_count: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Read error");
        // Split on ASCII whitespace (fast and good for normal English text)
        word_count += line.split_ascii_whitespace().count() as u64;
    }

    let elapsed = start.elapsed();
    println!("Word count: {}", word_count);
    println!("Execution time: {:.2?} seconds", elapsed.as_secs_f64());
}
