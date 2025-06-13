package main

import (
    "bufio"
    "fmt"
    "os"
    "strings"
    "time"
)

func main() {
    start := time.Now()
    file, err := os.Open("../largefile.txt")
    if err != nil {
        panic(err)
    }
    defer file.Close()

    count := 0
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        count += len(strings.Fields(scanner.Text()))
    }

    elapsed := time.Since(start)
    fmt.Printf("Word count: %d\n", count)
    fmt.Printf("Execution time: %.2f seconds\n", elapsed.Seconds())
}
