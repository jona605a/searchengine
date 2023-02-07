import java.io.*;
import java.util.Scanner;
 
class Index4 {
 
    int n = 100_000_007;
    WikiItem[] wikiItems = new WikiItem[n];


    private class ArticleItem {
        String title;
        ArticleItem next;

        ArticleItem(String s, ArticleItem n) {
            title = s;
            next = n;
        }
    }
 
    private class WikiItem {
        String str;
        ArticleItem articlelist;
 
        WikiItem(String s, ArticleItem a) {
            str = s;
            articlelist = a;
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
        word = input.next();
        title = word.substring(0,word.length()-1); // Assume that the first word is a title
        
        int collisionCounter = 0;
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
                wikiItems[hash] = new WikiItem(word, newArticle);
            } else {
                if (!wikiItems[hash].str.equals(word)) {
                    collisionCounter++;
                    System.out.println("Collision nr " + collisionCounter + "! Between words " + word + " and " + wikiItems[hash].str);
                    System.out.println(word.hashCode() + " and " + wikiItems[hash].str.hashCode());
                }

                if (!wikiItems[hash].articlelist.title.equals(title)) {
                    newArticle = new ArticleItem(title, wikiItems[hash].articlelist);
                    wikiItems[hash].articlelist = newArticle;
                }
            }
            
        }
        input.close();
    }
 
    public ArticleItem search(String searchstr) {
        int hash = hashString(searchstr);
        if (wikiItems[hash] == null) {
            return null;
        }
        return wikiItems[hash].articlelist;
    }
    
    public static void main(String[] args) {
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
                for (ArticleItem current = titles; current != null && current.title != null; current=current.next) {
                    System.out.print(current.title + " ");
                }
                System.out.println("\n");
            }
        }
        console.close();
    }
}