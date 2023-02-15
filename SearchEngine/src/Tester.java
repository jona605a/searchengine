public class Tester {
    public static void main(String[] args) throws Exception {
        if (args.length <= 2) {
            System.out.println("Test expects the command line arguments: [filename] [i] [j] , where i and j are index numbers to compare");
            return;
        }
        String filename = args[0];
        Index i1 = interpretIdx(filename, args[1]);
        Index i2 = interpretIdx(filename, args[2]);

        Index.WikiItem uWords1 = i1.getUniqueWords();
        Index.WikiItem uWords2 = i2.getUniqueWords();

        int l1 = 0;
        for (;uWords1 != null; uWords1=uWords1.next) {
            l1++;
        }
        int l2 = 0;
        for (;uWords2 != null; uWords2=uWords2.next) {
            l2++;
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

