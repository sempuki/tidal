import java.util.Iterator;
import java.util.NoSuchElementException;

// A doubly linked list with an iterator, a reverse walk, and removal of
// the elements that match a predicate.
public class Chain {
    static final class Node<T> {
        T value;
        Node<T> prev, next;
        Node(T value) { this.value = value; }
    }

    static final class LinkedSeq<T> implements Iterable<T> {
        private Node<T> head, tail;
        private int size;

        void addLast(T value) {
            Node<T> n = new Node<>(value);
            if (tail == null) { head = tail = n; }
            else { tail.next = n; n.prev = tail; tail = n; }
            size++;
        }

        void addFirst(T value) {
            Node<T> n = new Node<>(value);
            if (head == null) { head = tail = n; }
            else { n.next = head; head.prev = n; head = n; }
            size++;
        }

        int removeIf(java.util.function.Predicate<T> p) {
            int removed = 0;
            for (Node<T> n = head; n != null; n = n.next) {
                if (!p.test(n.value)) continue;
                if (n.prev != null) n.prev.next = n.next; else head = n.next;
                if (n.next != null) n.next.prev = n.prev; else tail = n.prev;
                size--;
                removed++;
            }
            return removed;
        }

        int size() { return size; }

        public Iterator<T> iterator() {
            return new Iterator<T>() {
                private Node<T> at = head;
                public boolean hasNext() { return at != null; }
                public T next() {
                    if (at == null) throw new NoSuchElementException();
                    T v = at.value;
                    at = at.next;
                    return v;
                }
            };
        }

        T lastMatching(java.util.function.Predicate<T> p) {
            for (Node<T> n = tail; n != null; n = n.prev) if (p.test(n.value)) return n.value;
            return null;
        }
    }

    public static void main(String[] args) {
        LinkedSeq<Integer> seq = new LinkedSeq<>();
        for (int i = 1; i <= 6; i++) seq.addLast(i * 10);
        seq.addFirst(5);
        int sum = 0;
        for (int v : seq) sum += v;
        int failures = 0;
        if (sum != 215) failures++;
        if (seq.removeIf(v -> v % 20 == 0) != 3) failures++;
        if (seq.size() != 4) failures++;
        StringBuilder order = new StringBuilder();
        for (int v : seq) order.append(v).append(' ');
        if (!order.toString().equals("5 10 30 50 ")) failures++;
        if (seq.lastMatching(v -> v < 40) != 30) failures++;
        System.exit(failures);
    }
}
