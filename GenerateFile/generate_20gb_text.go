package main

import (
    "bufio"
    "math/rand"
    "os"
    "strings"
    "time"
)

var words = []string{
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
}

func main() {
    const targetSizeGB = 20
    const targetSizeBytes = targetSizeGB * 1024 * 1024 * 1024
    const wordsPerLine = 1000

    rand.Seed(time.Now().UnixNano())
    outFile, err := os.Create("largefile_go.txt")
    if err != nil {
        panic(err)
    }
    defer outFile.Close()

    writer := bufio.NewWriterSize(outFile, 8*1024*1024)
    var bytesWritten int64

    progressStep := int64(1024 * 1024 * 1024) // 1GB

    for bytesWritten < targetSizeBytes {
        // Generate a line of random words
        lineWords := make([]string, wordsPerLine)
        for i := 0; i < wordsPerLine; i++ {
            lineWords[i] = words[rand.Intn(len(words))]
        }
        line := strings.Join(lineWords, " ") + "\n"
        n, _ := writer.WriteString(line)
        bytesWritten += int64(n)
        if bytesWritten%progressStep < int64(len(line)) {
            // Progress update every ~1GB
            println(float64(bytesWritten)/(1024*1024*1024), "GB written...")
        }
    }
    writer.Flush()
    println("Done!")
}
