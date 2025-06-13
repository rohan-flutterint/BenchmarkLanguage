import random
import os

words = [
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
    "open", "close", "push", "pull", "lift", "drop", "catch", "throw", "hold", "release"
]

target_size_gb = 20
target_size_bytes = target_size_gb * 1024 * 1024 * 1024  # 20GB
output_file = "largefile_python.txt"

with open(output_file, "w", encoding="utf-8") as f:
    bytes_written = 0
    while bytes_written < target_size_bytes:
        # Write a batch of words per line for better speed
        line_words = [random.choice(words) for _ in range(1000)]
        line = " ".join(line_words) + "\n"
        f.write(line)
        bytes_written += len(line.encode("utf-8"))
        if bytes_written % (1024*1024*1024) < 100000:  # Print progress every ~1GB
            print(f"{bytes_written / (1024*1024*1024):.2f} GB written...", end='\r')

print("Done!")
