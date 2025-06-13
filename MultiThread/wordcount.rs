use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;

fn count_words_in_chunk(filename: &str, start: u64, end: u64) -> usize {
    let mut file = File::open(filename).expect("open failed");
    file.seek(SeekFrom::Start(start)).expect("seek failed");
    let mut buf = vec![0u8; (end - start) as usize];
    file.read_exact(&mut buf).expect("read failed");
    let text = String::from_utf8_lossy(&buf);

    // For all except the first chunk, skip to the first whitespace (to avoid partial word)
    let slice = if start != 0 {
        match text.find(|c: char| c.is_whitespace()) {
            Some(pos) => &text[pos..],
            None => "",
        }
    } else {
        &text
    };

    slice.split_whitespace().count()
}

fn main() {
    let filename = "../largefile.txt";
    let metadata = std::fs::metadata(filename).expect("metadata failed");
    let file_size = metadata.len();
    let n_threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut handles = Vec::new();

    let chunk_size = (file_size as usize + n_threads - 1) / n_threads;

    let mut start: u64 = 0;
    for _ in 0..n_threads {
        let end = std::cmp::min(start + chunk_size as u64, file_size);
        let fname = filename.to_string();
        handles.push(thread::spawn(move || count_words_in_chunk(&fname, start, end)));
        start = end;
        if start >= file_size {
            break;
        }
    }

    let now = std::time::Instant::now();
    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    let elapsed = now.elapsed();

    println!("Word count: {}", total);
    println!("Execution time: {:.2} seconds", elapsed.as_secs_f64());
}
