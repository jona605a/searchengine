import java.util.HashSet;
import java.util.Date;

public class Tester {
    public static void main(String[] args) throws Exception {
        if (args.length <= 2) {
            System.out.println("Test expects the command line arguments: [filename] [i] [j] [testetype] , where i and j are index numbers to compare");
            return;
        }

        if(args[3].equals("correctness")){
            correctnessTest(args);
        }
        
        if(args[3].equals("time")){
            timeTest(args);
        }

    }

    public static void correctnessTest (String[] args) throws Exception {
        
        //initizise
        String filename = args[0];
        Index i1 = interpretIdx(filename, args[1]);
        Index i2 = interpretIdx(filename, args[2]);

        Index.WikiItem uWords1 = i1.getUniqueWords();
        Index.WikiItem uWords2 = i2.getUniqueWords();

        HashSet<String> words1 = new HashSet<String>();
        HashSet<String> words2 = new HashSet<String>();

        //Count Uniqe words and add them to a hashmap
        int l1 = 0;
        for (;uWords1 != null; uWords1=uWords1.next) {
            words1.add(uWords1.str);
            l1++;

        }
        int l2 = 0;
        for (;uWords2 != null; uWords2=uWords2.next) {
            words2.add(uWords2.str);
            l2++;
        }

        if (l1!=l2) {
            throw new Exception("Index"+args[1]+" encounters "+l1+" unique words, but Index"+args[2]+" encounters "+l2+".");
        }

        //run though all elements in Index args[1]
        for (String str : words1) {
            
            //Test that they have the same uniqe words
            if (!words2.contains(str)) {
                throw new Exception(str + "not in Index"+args[2]);
            }
            
            // Test that they have the same articles
            Index.ArticleItem a1 = i1.search(str);
            Index.ArticleItem a2 = i2.search(str);
                    
            while (a1 != null){
                if (!a1.str.equals(a2.str)) {
                    throw new Exception("Article "+a1.str+" is not "+a2.str+" for word\t"+str);
                }
                a1=a1.next;
                a2=a2.next;
            }
        }

        System.out.println("Test passed!");
        
    }

    public static void timeTest(String[] args) {
        String filename = args[0];
        
        long startTime = System.currentTimeMillis();
        Index i1 = interpretIdx(filename, args[1]);
        long TimeIndex1 = (new Date()).getTime() - startTime;

        startTime = System.currentTimeMillis();
        Index i2 = interpretIdx(filename, args[2]);
        long TimeIndex2 = (new Date()).getTime() - startTime;


        Index.WikiItem uWords1 = i1.getUniqueWords();
        Index.WikiItem uWords2 = i2.getUniqueWords();


        startTime = System.currentTimeMillis();
        for(Index.WikiItem item = uWords1; item != null; item = item.next) {
            Index.ArticleItem a1 = i1.search(item.str);
        }
        long TimeSearch1 = (new Date()).getTime() - startTime;

        startTime = System.currentTimeMillis();
        for(Index.WikiItem item = uWords2; item != null; item = item.next) {
            Index.ArticleItem a1 = i1.search(item.str);
        }
        long TimeSearch2 = (new Date()).getTime() - startTime;

        System.out.println("IndexTime Index"+args[1]+": \t"+TimeIndex1 + "ms");
        System.out.println("IndexTime Index"+args[1]+": \t"+TimeSearch1 + "ms \n");
        
        System.out.println("IndexTime Index"+args[2]+": \t"+TimeIndex2 + "ms");
        System.out.println("IndexTime Index"+args[2]+": \t"+TimeSearch2 + "ms");

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

