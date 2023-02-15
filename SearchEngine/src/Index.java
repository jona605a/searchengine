public interface Index {

    public class ArticleItem {
        String str;
        ArticleItem next;

        ArticleItem(String s, ArticleItem n) {
            str = s;
            next = n;
        }
    }

    public class WikiItem {
        String str;
        WikiItem next;
        ArticleItem articlelist;
 
        WikiItem(String s, WikiItem n, ArticleItem a) {
            str = s;
            next = n;
            articlelist = a;
        }
    }

    public ArticleItem search(String searchstr);

    public WikiItem getUniqueWords();

}
