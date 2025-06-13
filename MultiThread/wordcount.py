import os
import multiprocessing as mp
import mmap

def count_words_in_chunk(filename, start, end):
    with open(filename, 'r', encoding='utf-8') as f:
        # Memory-map the file
        mm = mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ)
        # Move to the start offset
        mm.seek(start)
        if start != 0:
            # Move to the next whitespace to avoid splitting a word
            while mm.tell() < end:
                if chr(mm.read_byte()).isspace():
                    break
        actual_start = mm.tell()
        mm.seek(actual_start)
        size = min(end, mm.size())
        text = mm.read(size - actual_start)
        # Only read up to `end`
        if mm.tell() > end:
            text = text[:end - actual_start]
        # Count words in this chunk
        word_count = len(text.decode('utf-8', errors='ignore').split())
        mm.close()
        return word_count

def parallel_word_count(filename, n_processes=None):
    if n_processes is None:
        n_processes = os.cpu_count() or 2
    filesize = os.path.getsize(filename)
    # Divide file into chunks
    chunk_size = filesize // n_processes
    offsets = []
    for i in range(n_processes):
        start = i * chunk_size
        end = filesize if i == n_processes - 1 else (i + 1) * chunk_size
        offsets.append((start, end))

    with mp.Pool(n_processes) as pool:
        results = pool.starmap(count_words_in_chunk, [(filename, start, end) for (start, end) in offsets])
    return sum(results)

if __name__ == "__main__":
    import time
    filename = "../largefile.txt"
    start = time.time()
    total = parallel_word_count(filename)
    end = time.time()
    print("Word count:", total)
    print("Execution time: %.2f seconds" % (end - start))
