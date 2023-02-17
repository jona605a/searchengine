import java.util.Arrays;
import java.util.HashMap;

public class Tester {
    public static void main(String[] args) throws Exception {
        if (args.length <= 2) {
            System.out.println("Test expects the command line arguments: [filename] [i] [j] , where i and j are index numbers to compare");
            return;
        }
        String filename = args[0];
        System.out.println(Arrays.toString(args));

        Index[] r = memoryTest(args);
        Index i1 = r[0];
        Index i2 = r[1];
        
        // Call getUniqueWords

        Index.WikiItem uWords1 = i1.getUniqueWords();
        Index.WikiItem uWords2 = i2.getUniqueWords();

        System.out.println("Done finding unique words");

        HashMap<String,Integer> words1 = new HashMap<String,Integer>();
        HashMap<String,Integer> words2 = new HashMap<String,Integer>();

        int l1 = 0;
        for (;uWords1 != null; uWords1=uWords1.next) {
            words1.computeIfPresent(uWords1.str, (str,x) -> x+1);
            words1.computeIfAbsent(uWords1.str, (str) -> 1);
            l1++;

        }
        int l2 = 0;
        for (;uWords2 != null; uWords2=uWords2.next) {
            words2.computeIfPresent(uWords2.str, (str,x) -> x+1);
            words2.computeIfAbsent(uWords2.str, (str) -> 1);
            l2++;
        }

        System.out.println("Number of unique words in "+filename+" : " + l1);

        for (String str : words1.keySet()) {
            if (words2.get(str) != words1.get(str)) {
                System.out.println(words1.get(str)+" is not "+words2.get(str)+" for word "+str);
            }
            Index.ArticleItem a1 = i1.search(str);
            Index.ArticleItem a2 = i2.search(str);
            // System.out.println(a1 + " " + a2 + "\t" + str);
            // System.out.println(a1.next + " " + a2.next);
            // Assume articleitems must be in the same order (file-read order)
            while (a1 != null){
                if (!a1.str.equals(a2.str)) {
                    System.out.println("Article "+a1.str+" is not "+a2.str+" for word\t"+str);
                }
                a1=a1.next;
                a2=a2.next;
            }
        }

        if (l1!=l2) {
            throw new Exception("Index"+args[1]+" encounters "+l1+" unique words, but Index"+args[2]+" encounters "+l2+".");
        }

        // for (;uWords1 != null; uWords1=uWords1.next) {
        //     if (!uWords1.str.equals(uWords2.str)) {
        //         throw new Exception("Encountered two words that differed. At "+uWords1.str+" and "+uWords2.str);
        //     }
        //     uWords2=uWords2.next;
        // }
        System.out.println("Test passed!");
    }


    public static Index[] memoryTest(String[] args) {
        String filename = args[0];
        
        // Prepare memory test
        System.gc();
        Runtime runtime = Runtime.getRuntime();

        // Memory test i1
        long memory1before = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Before running index"+args[1]+". Memory before: "+memory1before);
        
        Index i1 = interpretIdx(filename, args[1]);
        
        long memory1after  = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  running index"+args[1]+". Memory after:  "+memory1after);
        
        // Prepare memory test
        System.gc();
        
        // Memory test i2
        long memory2before = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Index"+args[1]+" initial memory usage: "+ (memory1after-memory1before) + ". And \"actual\" memory usage: " + (memory2before-memory1before));
        System.out.println("Before running index"+args[2]+". Memory before: "+memory2before);
        
        Index i2 = interpretIdx(filename, args[2]);
        
        long memory2after  = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  running index"+args[2]+". Memory after:  "+memory2after);
        
        System.gc();
        long memory2aftergc  = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("After  garbage index"+args[2]+". Memory after:  "+memory2aftergc);
        System.out.println("Index"+args[2]+" initial memory usage: "+ (memory2after-memory2before) + ". And \"actual\" memory usage: " + (memory2aftergc-memory2before));
        
        Index[] r = new Index[2];
        r[0] = i1; r[1] = i2;
        return r;
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
            //     return new Index6(filename);
        
            // case "7":
            //     return new Index7(filename);
        
            default:
                return null;
        }
    }
}

