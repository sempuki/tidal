// tidal-rules: LAM-10 MUL-7 MUL-8 CONV-2 CMP-15 NEW-5
// tidal-expect: run
// tidal-note: Port suite P2, Java persona, program buckets.cpp (reports/clean-room/P2/report.md). The rules are the ones the report names for the program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054). The first try is the same program.
// A hash map from string to int with separate chaining and a hasher
// that the caller supplies. It grows when the load passes 3/4.
// Port of Buckets.java, with no #include.

// A string key. Java's String.equals compares characters. In C++ two
// const char* compare by address (CMP-15), so the key wraps the pointer
// and compares the characters itself.
struct Str {
  const char* chars;
  Str(const char* c) : chars(c) {}
  bool operator==(const Str& other) const {
    const char* a = chars;
    const char* b = other.chars;
    while (*a != '\0' && *a == *b) { ++a; ++b; }
    return *a == *b;
  }
};

// A growable list, in place of java.util.ArrayList. It owns its array.
template <class T> class List {
  T* items = nullptr;
  int count = 0;
  int capacity = 0;
public:
  List() = default;
  List(const List&) = delete;             // NEW-P2: one owner per array
  List& operator=(const List&) = delete;
  ~List() { delete[] items; }
  void add(T value) {
    if (count == capacity) {
      int bigger = capacity == 0 ? 4 : capacity * 2;
      T* next = new T[bigger];
      for (int i = 0; i < count; i++) next[i] = items[i];
      delete[] items;
      items = next;
      capacity = bigger;
    }
    items[count++] = value;
  }
  int size() const { return count; }
  T& operator[](int i) { return items[i]; }
  T* begin() { return items; }
  T* end() { return items + count; }
};

template <class K, class V> struct Entry {
  const K key;
  V value;
  Entry(K k, V v) : key(k), value(v) {}
};

template <class K, class V> class ChainMap {
public:
  using Hasher = int (*)(const K&);       // LAM-10: a lambda with no captures
private:
  List<Entry<K, V>*>* buckets;
  int bucketCount;
  Hasher hasher;
  int count = 0;

  // Math.floorMod: % keeps the sign of the left operand (MUL-7).
  int index(const K& key, int n) const {
    int r = hasher(key) % n;
    return r < 0 ? r + n : r;
  }

  void grow() {
    List<Entry<K, V>*>* old = buckets;
    int oldCount = bucketCount;
    bucketCount = oldCount * 2;
    buckets = new List<Entry<K, V>*>[bucketCount];
    for (int i = 0; i < oldCount; i++)
      for (Entry<K, V>* e : old[i])
        buckets[index(e->key, bucketCount)].add(e);
    delete[] old;
  }

public:
  explicit ChainMap(Hasher h)
      : buckets(new List<Entry<K, V>*>[4]), bucketCount(4), hasher(h) {}
  ChainMap(const ChainMap&) = delete;
  ChainMap& operator=(const ChainMap&) = delete;
  ~ChainMap() {
    for (int i = 0; i < bucketCount; i++)
      for (Entry<K, V>* e : buckets[i]) delete e;
    delete[] buckets;
  }

  void put(const K& key, const V& value) {
    List<Entry<K, V>*>& chain = buckets[index(key, bucketCount)];
    for (Entry<K, V>* e : chain) {
      if (e->key == key) { e->value = value; return; }
    }
    chain.add(new Entry<K, V>(key, value));
    count++;
    if (count * 4 > bucketCount * 3) grow();
  }

  // Java returns null for a missing key. Here a null pointer does.
  V* get(const K& key) {
    for (Entry<K, V>* e : buckets[index(key, bucketCount)]) {
      if (e->key == key) return &e->value;
    }
    return nullptr;
  }

  int size() const { return count; }
  int buckets_() const { return bucketCount; }
};

int main() {
  // Java's int arithmetic wraps. Signed overflow in C++ is undefined
  // (MUL-10, ADD-9), so the hash runs in unsigned (MUL-8) and converts
  // back to int at the end (CONV-2).
  ChainMap<Str, int> counts([](const Str& s) {
    unsigned h = 0;
    for (const char* p = s.chars; *p != '\0'; ++p) h = h * 31u + (unsigned)*p;
    return (int)h;
  });
  const char* words[] = {"pear", "fig", "pear", "plum", "fig",
                         "pear", "kiwi", "lime", "date"};
  for (const char* w : words) {
    int* c = counts.get(w);
    counts.put(w, c == nullptr ? 1 : *c + 1);
  }
  int failures = 0;
  if (*counts.get("pear") != 3) failures++;
  if (*counts.get("fig") != 2) failures++;
  if (*counts.get("plum") != 1) failures++;
  if (counts.get("grape") != nullptr) failures++;
  if (counts.size() != 6) failures++;
  if (counts.buckets_() != 8) failures++;
  return failures;
}
