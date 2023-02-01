import java.io.*;
import java.util.Scanner;
 
class Index1 {
 
    WikiItem start;
 
    private class WikiItem {
        String str;
        WikiItem next;
 
        WikiItem(String s, WikiItem n) {
            str = s;
            next = n;
        }
    }
 
    public Index1(String filename) {
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
 
    public boolean search(String searchstr) {
        WikiItem current = start;
        while (current != null) {
            if (current.str.equals(searchstr)) {
                return true;
            }
            current = current.next;
        }
        return false;
    }
 
    public static void main(String[] args) {
        System.out.println("Preprocessing " + args[0]);
        Index1 i = new Index1(args[0]);
        Scanner console = new Scanner(System.in);
        for (;;) {
            System.out.println("Input search string or type exit to stop");
            String searchstr = console.nextLine();
            if (searchstr.equals("exit")) {
                break;
            }
            if (i.search(searchstr)) {
                System.out.println(searchstr + " exists");
            } else {
                System.out.println(searchstr + " does not exist");
            }
        }
        console.close();
    }
}