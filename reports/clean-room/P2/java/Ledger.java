import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

// Records with structural equality and hashing, held by two owners that
// share them. A change through one owner is seen through the other.
public class Ledger {
    record Point(int x, int y) {}
    record Segment(Point from, Point to) {
        int lengthSquared() {
            int dx = to.x() - from.x(), dy = to.y() - from.y();
            return dx * dx + dy * dy;
        }
    }

    static final class Account {
        final String name;
        int balance;
        Account(String name, int balance) { this.name = name; this.balance = balance; }
    }

    static final class Book {
        final List<Account> accounts = new ArrayList<>();
        void add(Account a) { accounts.add(a); }
        int total() { int t = 0; for (Account a : accounts) t += a.balance; return t; }
    }

    public static void main(String[] args) {
        int failures = 0;
        Segment a = new Segment(new Point(0, 0), new Point(3, 4));
        Segment b = new Segment(new Point(0, 0), new Point(3, 4));
        Segment c = new Segment(new Point(3, 4), new Point(0, 0));
        if (!a.equals(b)) failures++;
        if (a.equals(c)) failures++;
        if (a.lengthSquared() != 25) failures++;
        Set<Segment> seen = new HashSet<>(List.of(a, b, c));
        if (seen.size() != 2) failures++;

        Account shared = new Account("shared", 100);
        Book home = new Book(), work = new Book();
        home.add(shared);
        home.add(new Account("home", 40));
        work.add(shared);
        work.accounts.get(0).balance += 25;
        if (home.total() != 165) failures++;
        if (work.total() != 125) failures++;
        System.exit(failures);
    }
}
