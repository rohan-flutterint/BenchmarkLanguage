import java.io.*;
import java.util.concurrent.*;
import java.nio.charset.StandardCharsets;

public class WordCount {
    public static void main(String[] args) throws Exception {
        final int nThreads = Runtime.getRuntime().availableProcessors();
        final String filename = "../largefile.txt";
        final BlockingQueue<String> queue = new ArrayBlockingQueue<>(2000);
        final long[] counts = new long[nThreads];
        final BufferedReader reader = new BufferedReader(
            new InputStreamReader(new FileInputStream(filename), StandardCharsets.UTF_8),
            16 * 1024 * 1024
        );

        ExecutorService pool = Executors.newFixedThreadPool(nThreads);
        CountDownLatch latch = new CountDownLatch(nThreads);

        // Workers
        for (int i = 0; i < nThreads; i++) {
            final int idx = i;
            pool.submit(() -> {
                long local = 0;
                try {
                    while (true) {
                        String line = queue.poll(1, TimeUnit.SECONDS);
                        if (line == null) break;
                        local += line.trim().isEmpty() ? 0 : line.trim().split("\\s+").length;
                    }
                } catch (InterruptedException ignored) {}
                counts[idx] = local;
                latch.countDown();
            });
        }

        long start = System.nanoTime();

        // Producer
        String line;
        while ((line = reader.readLine()) != null) {
            queue.put(line);
        }
        reader.close();

        // Stop signals
        for (int i = 0; i < nThreads; i++) queue.put(""); // Empty line = poison pill

        latch.await();
        pool.shutdown();

        long total = 0;
        for (long c : counts) total += c;

        double elapsed = (System.nanoTime() - start) / 1e9;
        System.out.println("Word count: " + total);
        System.out.printf("Execution time: %.2f seconds\n", elapsed);
    }
}
