import java.io.*;
import java.util.Scanner;
 
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

    private class LinkedList {
        String str;
        LinkedList next;

        LinkedList(String s, LinkedList l) {
            str = s;
            next = l;
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
 
    public LinkedList search(String searchstr) {
        String title = start.str;
        title = title.substring(0, title.length()-1); // Remove "."
        LinkedList titles = new LinkedList(null, null);

        for (WikiItem current = start; current != null; current = current.next) {
            if (current.str.equals(searchstr) && (titles.next == null || !titles.str.equals(title))) {
                LinkedList tmp = new LinkedList(title, titles);
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
            LinkedList titles = i.search(searchstr);
            if (titles.next == null) {
                System.out.println(searchstr + " does not exist");
            } else {
                System.out.print("\""+searchstr+"\"" + " exists in the following articles:\n   ");
                for (LinkedList current = titles; current != null && current.str != null; current=current.next) {
                    System.out.print(current.str + " ");
                }
                System.out.println("\n");
            }
        }
        console.close();
    }
}