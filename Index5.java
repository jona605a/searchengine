import java.io.*;
import java.util.Scanner;
 
class Index5 {
 
    int n = 1_000; //Size of hash table
    int uniqeWords = 0;
    WikiItem[] wikiItems = new WikiItem[n]; // Hash table

    private class ArticleItem {        
    /*
    Objects used to create linked list of article titles 
    */
        String title;
        ArticleItem next;

        ArticleItem(String s, ArticleItem n) {
            title = s;
            next = n;
        }
    }
 
    private class WikiItem {
    /*
    Objects used to store words hased to same value in hash table as linked list  
    */
        String word;
        ArticleItem articlelist;
        WikiItem next;
 
        WikiItem(String s, ArticleItem a, WikiItem w) {
            word = s;
            articlelist = a;
            next = w;
        }
    }

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
 
    public Index5(String filename) {
        String word, title, previousWord;
        Scanner input;
        try {
            input = new Scanner(new File(filename), "UTF-8");
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
            return;
        }
        word = input.next();
        title = word.substring(0,word.length()-1); // Assume that the first word is a title
        
        // int collisionCounter = 0;
        while (input.hasNext()) {   // Read all words in input
            previousWord = word;
            word = input.next();
            
            // Update the current title
            if (previousWord.equals("---END.OF.DOCUMENT---")) {
                title = word.substring(0,word.length()-1);
            }

            insertWord(word, title);

            if(n <= uniqeWords){ //double space in hash table
                rehash();
                System.out.println(n);
            }
        }
        input.close();
    }
 
    public void insertWord(String word, String title){
        //int hash = word.hashCode();
        int hash = hashString(word);
        ArticleItem newArticle;
        // Checks if place in hashtable is empty
        if (wikiItems[hash] == null) {
            newArticle = new ArticleItem(title, null);
            wikiItems[hash] = new WikiItem(word, newArticle, null);
            uniqeWords ++;
        // If not emty then go though linked list of word hashed to this hash value and find word 
        } else {
            WikiItem item = wikiItems[hash];
            for (; item != null; item = item.next) {
                if (item.word.equals(word)) {
                    // check if the article title already is there
                    if (!item.articlelist.title.equals(title)) {
                        newArticle = new ArticleItem(title, item.articlelist);
                        item.articlelist = newArticle;
                    }
                    break;
                }
            }
            //If article not in list put article in list
            if (item == null) {
                newArticle = new ArticleItem(title, null);
                WikiItem newItem = new WikiItem(word, newArticle, wikiItems[hash]);
                wikiItems[hash] = newItem;
                uniqeWords ++;
            }
            }
    }

    public void rehash(){
        n = n*2;
        WikiItem[] NewwikiItems = new WikiItem[n];

        for(int i = 0; i < n/2; i ++){
            if (wikiItems[i] == null){
                continue;
            }
            for(WikiItem item = wikiItems[i];item != null; item = item.next){
                int hash = hashString(item.word);
                item.next = NewwikiItems[hash];
                NewwikiItems[hash] = item;   
            }
        }
        wikiItems = NewwikiItems;
    }
    public ArticleItem search(String searchstr) {
        /*
        Returns the Article list of titles where searchstr is present. If searchstr is not present in any articles return null.
         */
        int hash = hashString(searchstr);
        if (wikiItems[hash] == null) {
            return null;
        }
        for (WikiItem item = wikiItems[hash]; item != null; item = item.next) {
            if (item.word.equals(searchstr)) {
                return wikiItems[hash].articlelist;
            }
        }
        return null;
    }
    
    public static void main(String[] args) {
       // Run test
        if (args.length > 1) {
            test(args);
            return;
        }
        // Else run main
        System.out.println("Preprocessing " + args[0]);
        Index5 i = new Index5(args[0]);
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
                for (ArticleItem current = titles; current != null && current.title != null; current=current.next) {
                    System.out.print(current.title + " ");
                }
                System.out.println("\n");
            }
        }
        console.close();
    }

    public static void test(String[] args) {
        System.out.println("Preprocessing " + args[0]);
        Index5 i = new Index5(args[0]);
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
}