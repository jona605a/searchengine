import java.io.*;
import java.util.*;
//import java.util.Scanner;
 
class Index2 {
 
    WikiItem start;
 
    private class WikiItem {
        String str;
        WikiItem next;
 
        WikiItem(String s, WikiItem n) {
            str = s;
            next = n;
        }
    }
 
    public Index2(String filename) {
        String word;
        WikiItem current, tmp;
        try {
            Scanner input = new Scanner(new File(filename), "UTF-8");
            word = input.next();
            start = new WikiItem(word, null);
            current = start;
            while (input.hasNext()) {   // Read all words in input
                word = input.next();
                System.out.println(word);
                tmp = new WikiItem(word, null);
                current.next = tmp;
                current = tmp;
            }
            input.close();
        } catch (FileNotFoundException e) {
            System.out.println("Error reading file " + filename);
        }
    }
 
    public List<String> search(String searchstr) {
        WikiItem current = start;
        String title = start.str;
        List<String> titles = new ArrayList<String>();
        while (current != null) {
            if (current.str.equals(searchstr)) {
                if (titles.size() == 0 || titles.get(titles.size()-1) != title) {
                    titles.add(title.substring(0,title.length()-1));
                }
            } else if (current.str.equals("---END.OF.DOCUMENT---")) {
                if (current.next != null) {
                    title = current.next.str;
                }
            }
            current = current.next;
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