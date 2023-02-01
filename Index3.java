import java.io.*;
import java.util.*;
//import java.util.Scanner;
 
class Index3 {
 
    WikiItem start;

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
        WikiItem next;
        ArticleItem articlelist;
 
        WikiItem(String s, WikiItem n, ArticleItem a) {
            str = s;
            next = n;
            articlelist = a;
        }
    }
 
    public Index3(String filename) {
        String word, title, previousWord;
        WikiItem tmp, newItem;
        ArticleItem newArticle;
        try {
            Scanner input = new Scanner(new File(filename), "UTF-8");
            word = input.next();
            start = new WikiItem(word, null, null);
            title = word.substring(0,word.length()-1); // Assume that the first word is a title
            while (input.hasNext()) {   // Read all words in input
                previousWord = word;
                word = input.next();
                // System.out.println(word);
                
                // Update the current title
                if (previousWord.equals("---END.OF.DOCUMENT---")) {
                    title = word.substring(0,word.length()-1);
                    System.out.println(title);
                }

                // Find the word in the index (if it exists) and add the title to its articlelist
                for (tmp = start; tmp!=null; tmp=tmp.next) {
                    if (tmp.str.equals(word)){
                        ArticleItem tmp_article = tmp.articlelist;
                        for (; tmp_article!=null; tmp_article = tmp_article.next) {
                            if (tmp_article.title.equals(title)) {
                                break;
                            }
                        }

                        if (tmp_article == null) { // Title wasn't found
                            newArticle = new ArticleItem(title, tmp.articlelist);
                            tmp.articlelist = newArticle;
                        }
                        break;
                    }
                }

                // If the word was not in the index, add the new word. 
                if (tmp == null) {
                    newArticle = new ArticleItem(title, null);
                    newItem = new WikiItem(word, start, newArticle);
                    start = newItem;
                }
            }
            input.close();
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
        }
    }
 
    public List<String> search(String searchstr) {
        List<String> titles = new ArrayList<String>();

        for (WikiItem current = start; current != null; current = current.next) {
            if (current.str.equals(searchstr)) {
                for (ArticleItem ai = current.articlelist; ai!=null; ai=ai.next) {
                    titles.add(ai.title);
                }
                break;
            }
        }
        return titles;
    }
 
    public static void main(String[] args) {
        System.out.println("Preprocessing " + args[0]);
        Index3 i = new Index3(args[0]);
        Scanner console = new Scanner(System.in);
        while (true) {
            System.out.println("\nInput search string or type exit to stop:");
            String searchstr = console.nextLine();
            if (searchstr.equals("exit")) {
                break;
            }
            List<String> titles = i.search(searchstr);
            if (titles.isEmpty()) {
                System.out.println(searchstr + " does not exist");
            } else {
                System.out.println(searchstr + " exists in the following articles:");
                System.out.println(titles);
            }
        }
        console.close();
    }
}