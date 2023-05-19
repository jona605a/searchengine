import java.util.Arrays;
import java.util.HashSet;

public class Tester {
    public static void main(String[] args) throws Exception {
        if (args.length <= 3) {
            System.out.println(
                    "Test expects the command line arguments: [filename] [i] [j] [testtype] , where i and j are index numbers to compare");
            return;
        }

        switch (args[3]) {
            case "correctness":
                correctnessTest(args);
                break;

            case "timeIDX":
                timeTestIDX(args);
                break;

            case "timeFILE":
                timeTestFile(args);
                break;

            case "memory":
                memoryTest(args);
                break;

            default:
                break;
        }
        return;
    }

    public static void correctnessTest(String[] args) throws Exception {
        // Initialize
        System.out.println(Arrays.toString(args));

        Index i1 = interpretIdx(args[0], args[1]);
        Index i2 = interpretIdx(args[0], args[2]);

        // Call getUniqueWords
        Index.WikiItem uWords1 = i1.getUniqueWords();
        Index.WikiItem uWords2 = i2.getUniqueWords();

        System.out.println("Done finding unique words");

        // Count Unique words and add them to a hashset
        HashSet<String> words1 = new HashSet<String>();
        HashSet<String> words2 = new HashSet<String>();

        for (; uWords1 != null; uWords1 = uWords1.next) {
            words1.add(uWords1.str);
        }
        for (; uWords2 != null; uWords2 = uWords2.next) {
            words2.add(uWords2.str);
        }

        if (words1.size() != words2.size()) {
            throw new Exception("Error! Index" + args[1] + " encounters " + words1.size() + " unique words, but Index"
                    + args[2] + " encounters " + words2.size() + ".");
        }

        // run though all elements in Index args[1]
        for (String str : words1) {

            // Test that they have the same unique words
            if (!words2.contains(str)) {
                throw new Exception(str + "not in Index" + args[2]);
            }

            // Test that they have the same articles
            Index.ArticleItem a1 = i1.search(str);
            Index.ArticleItem a2 = i2.search(str);

            while (a1 != null) {
                if (!a1.str.equals(a2.str)) {
                    throw new Exception("Article " + a1.str + " is not " + a2.str + " for word\t" + str);
                }
                a1 = a1.next;
                a2 = a2.next;
            }
        }
        System.out.println("Test passed!");
    }

    public static void timeTestFile(String[] args) {
        String filename = args[0];
        long startTime;

        for (int i = 2; i != 6; i++) {
            startTime = System.currentTimeMillis();
            Index index = interpretIdx(filename, String.valueOf(i));
            long TimeIndex = System.currentTimeMillis() - startTime;

            System.out.println("IndexTime Index" + i + ": \t" + TimeIndex + "ms");

            Index.WikiItem uWords = index.getUniqueWords();

            startTime = System.currentTimeMillis();
            for (Index.WikiItem item = uWords; item != null; item = item.next) {
                /* Index.ArticleItem a1 = */ index.search(item.str);
            }
            long TimeSearch = System.currentTimeMillis() - startTime;

            System.out.println("SearchTime Index" + i + ": \t" + TimeSearch + "ms \n");
        }
    }

    public static void timeTestIDX(String[] args) {
        long startTime;

        String[] files = { "Data/WestburyLab.wikicorp.201004_100KB.txt",
                "Data/WestburyLab.wikicorp.201004_1MB.txt",
                "Data/WestburyLab.wikicorp.201004_2MB.txt",
                "Data/WestburyLab.wikicorp.201004_5MB.txt",
                "Data/WestburyLab.wikicorp.201004_10MB.txt",
                "Data/WestburyLab.wikicorp.201004_20MB.txt",
                "Data/WestburyLab.wikicorp.201004_50MB.txt",
                "Data/WestburyLab.wikicorp.201004_100MB.txt",
                "Data/WestburyLab.wikicorp.201004_200MB.txt",
                "Data/WestburyLab.wikicorp.201004_400MB.txt",
                "Data/WestburyLab.wikicorp.201004_800MB.txt" };

        for (int i = 0; i != 4; i++) {
            System.out.println(files[i]);

            startTime = System.currentTimeMillis();
            Index index = interpretIdx(files[4], String.valueOf(args[1]));
            long TimeIndex = System.currentTimeMillis() - startTime;

            System.out.println("IndexTime Index" + args[1] + ": \t" + TimeIndex);

            Index.WikiItem uWords = index.getUniqueWords();

            startTime = System.currentTimeMillis();
            for (Index.WikiItem item = uWords; item != null; item = item.next) {
                /* Index.ArticleItem a1 = */ index.search(item.str);
            }
            long TimeSearch = System.currentTimeMillis() - startTime;

            System.out.println("SearchTime Index" + i + ": \t" + TimeSearch + "\n");
        }
    }

    public static void memoryTest(String[] args) {
        String filename = args[0];

        // Prepare memory test
        System.gc();
        Runtime runtime = Runtime.getRuntime();

        // Memory test i1
        long memory1before = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Before running index" + args[1] + ". Memory before: " + memory1before);

        Index i1 = interpretIdx(filename, args[1]);

        long memory1after = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  running index" + args[1] + ". Memory after:  " + memory1after);

        // Prepare memory test
        System.gc();

        // Memory test i2
        long memory2before = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Index" + args[1] + " total memory usage: " + (memory1after - memory1before)
                + ". And \"actual\" memory usage: " + (memory2before - memory1before));
        System.out.println("Before running index" + args[2] + ". Memory before: " + memory2before);

        Index i2 = interpretIdx(filename, args[2]);

        long memory2after = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  running index" + args[2] + ". Memory after:  " + memory2after);

        System.gc();
        long memory2aftergc = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  garbage index" + args[2] + ". Memory after:  " + memory2aftergc);
        System.out.println("Index" + args[2] + " total memory usage: " + (memory2after - memory2before)
                + ". And \"actual\" memory usage: " + (memory2aftergc - memory2before));

        // Ignore errors
        i2 = i1;
        i1 = i2;
        return;
    }

    private static Index interpretIdx(String filename, String id) {
        switch (id) {
            case "2":
                return new Index2(filename);

            case "3":
                return new Index3(filename);

            case "4":
                return new Index4(filename);

            case "5":
                return new Index5(filename);

            // case "6":
            // return new Index6(filename);

            // case "7":
            // return new Index7(filename);

            default:
                return null;
        }
    }
}
