import java.io.*;
import java.util.Scanner;
 
class Index2 implements Index{
 
    WikiItem start; 
 
    public Index2(String filename) {
        String word;
        WikiItem current, tmp;
        Scanner input;
        try {
            input = new Scanner(new File(filename), "UTF-8");
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
            return;
        }
        word = input.next();
        start = new WikiItem(word, null, null);
        current = start;
        while (input.hasNext()) {   // Read all words in input
            word = input.next();
            // System.out.println(word);
            tmp = new WikiItem(word, null, null);
            current.next = tmp;
            current = tmp;
        }
        input.close();
    
    }
 
    @Override
    public ArticleItem search(String searchstr) {
        String title = start.str;
        title = title.substring(0, title.length()-1); // Remove "."
        ArticleItem titles = new ArticleItem(null, null);

        for (WikiItem current = start; current != null; current = current.next) {
            if (current.str.equals(searchstr) && (titles.next == null || !titles.str.equals(title))) {
                ArticleItem tmp = new ArticleItem(title, titles);
                titles = tmp;
            } else if (current.str.equals("---END.OF.DOCUMENT---") && current.next != null) {
                title = current.next.str;
                title = title.substring(0, title.length()-1); // Remove "."
            }
        }
        return titles;
    }
 
    public static void main(String[] args) {
        System.out.println("Preprocessing " + args[0]);
        Index2 i = new Index2(args[0]);
        Scanner console = new Scanner(System.in);
        while (true) {
            System.out.println("Input search string or type exit to stop");
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
        WikiItem uniqeWordsStart = new WikiItem(start.str, null,start.articlelist);
        WikiItem word, tmp, newUniqeWord;

        for(word=start; word!=null; word=word.next) {   // Go though all the words
            
            for (tmp = uniqeWordsStart; tmp!=null; tmp=tmp.next){ // Go though the linked list of uniqeWords
                if (tmp.str.equals(word.str)){ // The word already exist in the linked list uniqeWords
                    break;
                }
            }
            if (tmp == null) { // word wasn't found in the linked list uniqeWords
                newUniqeWord = new WikiItem(word.str,uniqeWordsStart,word.articlelist); //word is added as the head of uniqeWords
                uniqeWordsStart = newUniqeWord;
            } 
        }
        
        return uniqeWordsStart;
    }
}