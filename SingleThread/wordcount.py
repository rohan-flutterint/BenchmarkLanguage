import time

def word_count(filename):
    count = 0
    with open(filename, 'r', encoding='utf-8') as f:
        for line in f:
            count += len(line.split())
    return count

if __name__ == "__main__":
    start = time.time()
    result = word_count("../largefile.txt")
    end = time.time()
    print(f"Word count: {result}")
    print(f"Execution time: {end - start:.2f} seconds")
