import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.util.concurrent.TimeUnit;

public class WordCount {
    public static void main(String[] args) throws IOException {
        long startTime = System.nanoTime();
        String filename = "../largefile.txt";
        // 16 MB buffer
        BufferedReader reader = new BufferedReader(
                new InputStreamReader(
                        new FileInputStream(filename), StandardCharsets.UTF_8
                ),
                16 * 1024 * 1024
        );

        long wordCount = 0;
        String line;
        

        while ((line = reader.readLine()) != null) {
            // split by one or more whitespace
            wordCount += line.split("\\s+").length;
        }
        reader.close();

        long endTime = System.nanoTime();
        double elapsed = (endTime - startTime) / 1e9;
        System.out.println("Word count: " + wordCount);
        System.out.printf("Execution time: %.2f seconds\n", elapsed);
    }
}
