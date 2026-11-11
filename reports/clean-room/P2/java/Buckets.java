import java.util.ArrayList;
import java.util.List;

// A hash map from String to Integer with separate chaining and a hasher
// that the caller supplies. It grows when the load passes 3/4.
public class Buckets {
    interface Hasher<K> { int hash(K key); }

    static final class Entry<K, V> {
        final K key;
        V value;
        Entry(K key, V value) { this.key = key; this.value = value; }
    }

    static final class ChainMap<K, V> {
        private List<List<Entry<K, V>>> buckets = new ArrayList<>();
        private final Hasher<K> hasher;
        private int size = 0;

        ChainMap(Hasher<K> hasher) {
            this.hasher = hasher;
            for (int i = 0; i < 4; i++) buckets.add(new ArrayList<>());
        }

        private int index(K key, int n) { return Math.floorMod(hasher.hash(key), n); }

        void put(K key, V value) {
            for (Entry<K, V> e : buckets.get(index(key, buckets.size()))) {
                if (e.key.equals(key)) { e.value = value; return; }
            }
            buckets.get(index(key, buckets.size())).add(new Entry<>(key, value));
            size++;
            if (size * 4 > buckets.size() * 3) grow();
        }

        V get(K key) {
            for (Entry<K, V> e : buckets.get(index(key, buckets.size()))) {
                if (e.key.equals(key)) return e.value;
            }
            return null;
        }

        int size() { return size; }
        int bucketCount() { return buckets.size(); }

        private void grow() {
            List<List<Entry<K, V>>> old = buckets;
            buckets = new ArrayList<>();
            for (int i = 0; i < old.size() * 2; i++) buckets.add(new ArrayList<>());
            for (List<Entry<K, V>> chain : old)
                for (Entry<K, V> e : chain)
                    buckets.get(index(e.key, buckets.size())).add(e);
        }
    }

    public static void main(String[] args) {
        ChainMap<String, Integer> counts = new ChainMap<>(s -> {
            int h = 0;
            for (char c : s.toCharArray()) h = h * 31 + c;
            return h;
        });
        String[] words = {"pear", "fig", "pear", "plum", "fig", "pear", "kiwi", "lime", "date"};
        for (String w : words) {
            Integer c = counts.get(w);
            counts.put(w, c == null ? 1 : c + 1);
        }
        int failures = 0;
        if (counts.get("pear") != 3) failures++;
        if (counts.get("fig") != 2) failures++;
        if (counts.get("plum") != 1) failures++;
        if (counts.get("grape") != null) failures++;
        if (counts.size() != 6) failures++;
        if (counts.bucketCount() != 8) failures++;
        System.exit(failures);
    }
}
