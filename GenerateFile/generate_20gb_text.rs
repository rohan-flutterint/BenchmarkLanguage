use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

// Very simple linear congruential generator for fast, dependency-free random numbers
struct LcgRng {
    state: u64,
}
impl LcgRng {
    fn new(seed: u64) -> Self {
        LcgRng { state: seed }
    }
    fn next_usize(&mut self, max: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as usize % max
    }
}

fn main() {
    let words = [
        "apple", "banana", "orange", "grape", "lemon", "melon", "peach", "plum", "pear", "berry",
        "car", "truck", "bus", "train", "plane", "ship", "boat", "bike", "scooter", "van",
        "cat", "dog", "mouse", "horse", "sheep", "goat", "lion", "tiger", "bear", "wolf",
        "bird", "duck", "chicken", "eagle", "owl", "sparrow", "crow", "pigeon", "swan", "falcon",
        "house", "home", "apartment", "building", "school", "college", "office", "shop", "mall", "bank",
        "tree", "bush", "grass", "flower", "leaf", "root", "branch", "forest", "jungle", "park",
        "river", "lake", "ocean", "sea", "pond", "stream", "creek", "waterfall", "beach", "coast",
        "mountain", "hill", "valley", "cliff", "cave", "desert", "plain", "island", "field", "meadow",
        "sun", "moon", "star", "planet", "sky", "cloud", "storm", "rain", "snow", "wind",
        "fire", "earth", "rock", "stone", "metal", "gold", "silver", "copper", "iron", "steel",
        "phone", "laptop", "computer", "tablet", "camera", "radio", "tv", "clock", "watch", "speaker",
        "book", "magazine", "paper", "pen", "pencil", "eraser", "notebook", "bag", "bottle", "glass",
        "music", "song", "dance", "game", "sport", "movie", "show", "film", "art", "picture",
        "friend", "family", "mother", "father", "sister", "brother", "uncle", "aunt", "cousin", "child",
        "student", "teacher", "doctor", "nurse", "engineer", "artist", "actor", "singer", "writer", "driver",
        "happy", "sad", "angry", "excited", "tired", "sleepy", "hungry", "thirsty", "scared", "bored",
        "big", "small", "long", "short", "tall", "tiny", "large", "wide", "thin", "fat",
        "fast", "slow", "quick", "bright", "dark", "loud", "quiet", "noisy", "soft", "hard",
        "hot", "cold", "warm", "cool", "wet", "dry", "clean", "dirty", "fresh", "old",
        "young", "new", "early", "late", "easy", "difficult", "strong", "weak", "rich", "poor",
        "good", "bad", "better", "best", "worse", "worst", "right", "wrong", "left", "right",
        "open", "close", "start", "stop", "run", "walk", "jump", "sit", "stand", "lie",
        "work", "play", "read", "write", "speak", "listen", "think", "know", "learn", "teach",
        "love", "hate", "like", "dislike", "want", "need", "see", "look", "watch", "show",
        "give", "take", "send", "receive", "buy", "sell", "pay", "cost", "spend", "save",
        "cook", "eat", "drink", "sleep", "dream", "wake", "wash", "clean", "build", "break",
        "drive", "ride", "fly", "swim", "travel", "visit", "come", "go", "leave", "arrive",
        "open", "close", "push", "pull", "lift", "drop", "catch", "throw", "hold", "release",
    ];

    const TARGET_SIZE_GB: u64 = 2;
    const TARGET_SIZE_BYTES: u64 = TARGET_SIZE_GB * 1024 * 1024 * 1024;
    const WORDS_PER_LINE: usize = 1000;
    const PROGRESS_STEP: u64 = 1024 * 1024 * 1024; // 1GB

    let file = File::create("largefile_rust.txt").expect("Cannot create file");
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);

    let mut bytes_written: u64 = 0;
    let mut rng = LcgRng::new(42); // Fixed seed for repeatability

    let start = Instant::now();
    while bytes_written < TARGET_SIZE_BYTES {
        let mut line = String::with_capacity(WORDS_PER_LINE * 6);
        for i in 0..WORDS_PER_LINE {
            let word = words[rng.next_usize(words.len())];
            line.push_str(word);
            if i != WORDS_PER_LINE - 1 {
                line.push(' ');
            }
        }
        line.push('\n');
        writer.write_all(line.as_bytes()).unwrap();
        bytes_written += line.len() as u64;

        if bytes_written % PROGRESS_STEP < line.len() as u64 {
            println!("{:.2} GB written...", bytes_written as f64 / (1024.0 * 1024.0 * 1024.0));
        }
    }
    writer.flush().unwrap();
    println!("Done in {:.2?}", start.elapsed());
}
