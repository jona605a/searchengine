import java.io.*;
import java.util.Scanner;
 
class Index3 implements Index {
 
    WikiItem start;
 
    public Index3(String filename) {
        String word, title, previousWord;
        WikiItem tmp, newItem;
        ArticleItem newArticle;
        Scanner input;
        try {
            input = new Scanner(new File(filename), "UTF-8");
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
            return;
        }
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
            }

            // Find the word in the index (if it exists) and add the title to its articlelist
            for (tmp = start; tmp!=null; tmp=tmp.next) {
                if (tmp.str.equals(word)){
                    ArticleItem tmp_article = tmp.articlelist;
                    for (; tmp_article!=null; tmp_article = tmp_article.next) {
                        if (tmp_article.str.equals(title)) {
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
    
    }

    @Override
    public ArticleItem search(String searchstr) {
        ArticleItem titles = new ArticleItem(null, null);

        for (WikiItem current = start; current != null; current = current.next) {
            if (current.str.equals(searchstr)) {
                for (ArticleItem ai = current.articlelist; ai!=null; ai=ai.next) {
                    ArticleItem tmp = new ArticleItem(ai.str, titles);
                    titles = tmp;
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
            ArticleItem titles = i.search(searchstr);
            if (titles.next == null) {
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

    @Override
    public WikiItem getUniqueWords() {
        return start;
    }
}