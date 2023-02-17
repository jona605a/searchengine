import java.io.*;
import java.util.Scanner;
 
class Index5 implements Index{
 
    int n = 1_000; //Size of hash table
    int uniqeWords = 0;
    WikiItem[] wikiItems = new WikiItem[n]; // Hash table

    private int hashString(String s) {
        
        return (s.hashCode() % n + n) % n;
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
        word = "---END.OF.DOCUMENT---"; // Assume that the first word is a title
        title = "";
        
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
        int hash = hashString(word);
        ArticleItem newArticle;
        // Checks if place in hashtable is empty
        if (wikiItems[hash] == null) {
            newArticle = new ArticleItem(title, null);
            wikiItems[hash] = new WikiItem(word, null, newArticle);
            uniqeWords ++;
        // If not emty then go though linked list of word hashed to this hash value and find word 
        } else {
            WikiItem item = wikiItems[hash];
            for (; item != null; item = item.next) {
                if (item.str.equals(word)) {
                    // check if the article title already is there
                    if (!item.articlelist.str.equals(title)) {
                        newArticle = new ArticleItem(title, item.articlelist);
                        item.articlelist = newArticle;
                    }
                    break;
                }
            }
            // Hash existed, but word wasn't found (collision). Add the new word. 
            if (item == null) {
                newArticle = new ArticleItem(title, null);
                WikiItem newItem = new WikiItem(word, wikiItems[hash],newArticle);
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
            WikiItem item = wikiItems[i];
            while (item!=null) {
                int hash = hashString(item.str);
                WikiItem tmp = item.next;
                item.next = NewwikiItems[hash];
                NewwikiItems[hash] = item;
                item = tmp;
            }
        }
        wikiItems = NewwikiItems;
    }

    @Override
    public ArticleItem search(String searchstr) {
        /*
        Returns the Article list of titles where searchstr is present. If searchstr is not present in any articles return null.
         */
        int hash = hashString(searchstr);
        if (wikiItems[hash] == null) {
            return null;
        }
        for (WikiItem item = wikiItems[hash]; item != null; item = item.next) {
            if (item.str.equals(searchstr)) {
                return item.articlelist;
            }
        }
        return null;
    }
    
    public static void main(String[] args) {
       // Run test
        if (args.length > 1) {
            testCollisions(args);
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