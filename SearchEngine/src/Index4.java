import java.io.*;
import java.util.Scanner;
 
class Index4 implements Index{
 
    int n = 10_007; // Prime
    WikiItem[] wikiItems = new WikiItem[n];


    private int hashString(String s) {
        
        return (s.hashCode() % n + n) % n;
        
        // int base = 109;
        // int acc = 1;
        // double sum = 0;
        // for (int i = 0; i < s.length(); i++) {
        //     sum = sum + s.charAt(i)*acc;
        //     acc = acc*base;
        // }
        // return (((int) sum) % n + n) % n;
    }
 
    public Index4(String filename) {
        String word, title, previousWord;
        ArticleItem newArticle;
        Scanner input;
        try {
            input = new Scanner(new File(filename), "UTF-8");
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
            return;
        }
        word = "---END.OF.DOCUMENT---";
        title = "";

        // int collisionCounter = 0;
        while (input.hasNext()) {   // Read all words in input
            previousWord = word;
            word = input.next();
            // System.out.println(word);
            
            // Update the current title
            if (previousWord.equals("---END.OF.DOCUMENT---")) {
                title = word.substring(0,word.length()-1);
            }

            //int hash = word.hashCode();
            int hash = hashString(word);
            
            if (wikiItems[hash] == null) {
                newArticle = new ArticleItem(title, null);
                wikiItems[hash] = new WikiItem(word, null, newArticle);
            } else {
                WikiItem item = wikiItems[hash];
                for (; item != null; item = item.next) {
                    if (item.str.equals(word)) {
                        if (!item.articlelist.str.equals(title)) {
                            newArticle = new ArticleItem(title, item.articlelist);
                            item.articlelist = newArticle;
                        }
                        break;
                    }
                }
                if (item == null) {
                    newArticle = new ArticleItem(title, null);
                    WikiItem newItem = new WikiItem(word, wikiItems[hash],newArticle );
                    wikiItems[hash] = newItem;
                }

            }
            
        }
        input.close();
    }
    @Override
    public ArticleItem search(String searchstr) {
        int hash = hashString(searchstr);
        if (wikiItems[hash] == null) {
            return null;
        }
        for (WikiItem item = wikiItems[hash]; item != null; item = item.next) {
            if (item.str.equals(searchstr)) {
                return wikiItems[hash].articlelist;
            }
        }
        return null;
    }
    
    public static void main(String[] args) {
        if (args.length > 1) {
            testCollisions(args);
            return;
        }
        System.out.println("Preprocessing " + args[0]);
        Index4 i = new Index4(args[0]);
        Scanner console = new Scanner(System.in);
        while (true) {
            System.out.println("\nInput search string or type exit to stop:");
            String searchstr = console.nextLine();
            if (searchstr.equals("exit")) {
                break;
            }
            ArticleItem titles = i.search(searchstr);
            if (titles == null) {
                System.out.println(searchstr + " does not exist");
            } else {
                System.out.print("\""+searchstr+"\"" + " exists in the following articles:\n   ");
                for (ArticleItem current = titles; current != null && current.str != null; current=current.next) {
                    System.out.print(current.str + " ");
                }
                System.out.println("\n");
            }
        }
        console.close();
    }

    public static void testCollisions(String[] args) {
        System.out.println("Preprocessing " + args[0]);
        Index4 i = new Index4(args[0]);
        int[] sizes = new int[i.n];
        for (int j = 0; j < sizes.length; j++) {
            if (i.wikiItems[j] != null) {
                int count = 0;
                for (WikiItem item = i.wikiItems[j]; item!=null; item=item.next) {
                    count++;
                }
                sizes[j] = count;
            }
        }
        int unique = 0;
        int n_words = 0;
        for (int j = 0; j < sizes.length; j++) {
            if (sizes[j] > 0) {
                unique++;
                n_words+=sizes[j];
            }
        }
        System.out.println("Unique hashes: " + unique);
        System.out.println("Unique words: " + n_words);
    }

    @Override
    public WikiItem getUniqueWords() {
        WikiItem uniqeWordsStart = null;
        WikiItem word, newUniqeWord;
    
        for(int i = 0; i!=n; i++) {   // Go though the hashmap
            
            if(wikiItems[i] != null){
                for (word = wikiItems[i]; word!=null; word=word.next){ // Go though the linked list listed of words with hashvalue i
                    newUniqeWord = new WikiItem(word.str,uniqeWordsStart,word.articlelist); //word is added as the head of uniqeWords
                    uniqeWordsStart = newUniqeWord;
                }    
            }

        }
        
        return uniqeWordsStart;
    }
}