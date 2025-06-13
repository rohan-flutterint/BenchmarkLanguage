package main

import (
    "bufio"
    "fmt"
    "os"
    "runtime"
    "strings"
    "sync"
    "time"
)

func countWords(lines <-chan string, results chan<- int, wg *sync.WaitGroup) {
    defer wg.Done()
    count := 0
    for line := range lines {
        count += len(strings.Fields(line))
    }
    results <- count
}

func main() {
    start := time.Now() // Start timer

    filename := "../largefile.txt"
    file, err := os.Open(filename)
    if err != nil {
        panic(err)
    }
    defer file.Close()

    nWorkers := runtime.NumCPU()
    lines := make(chan string, 1000)
    results := make(chan int, nWorkers)
    var wg sync.WaitGroup

    // Start workers
    for i := 0; i < nWorkers; i++ {
        wg.Add(1)
        go countWords(lines, results, &wg)
    }

    // Read lines and send to workers
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines <- scanner.Text()
    }
    close(lines)
    wg.Wait()
    close(results)

    // Sum up results
    total := 0
    for c := range results {
        total += c
    }

    elapsed := time.Since(start) // End timer

    fmt.Println("Word count:", total)
    fmt.Printf("Execution time: %.2f seconds\n", elapsed.Seconds())
}
