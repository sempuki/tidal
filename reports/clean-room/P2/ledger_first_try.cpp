// Records with structural equality and hashing, held by two owners that
// share them. A change through one owner is seen through the other.
// Port of Ledger.java, with no #include.

// LSUP-1, LSUP-4: the shape that a braced list needs, declared by the
// program because no header is included.
namespace std {
using size_t = decltype(sizeof 0);
template <class E> class initializer_list {
  const E* first;
  size_t count;
  constexpr initializer_list(const E* f, size_t n) : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
};
}

// A growable list, in place of java.util.ArrayList.
template <class T> class List {
  T* items = nullptr;
  int count = 0;
  int capacity = 0;
public:
  List() = default;
  List(const List&) = delete;
  List& operator=(const List&) = delete;
  ~List() { delete[] items; }
  void add(const T& value) {
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
  T& get(int i) { return items[i]; }
  const T* begin() const { return items; }
  const T* end() const { return items + count; }
};

// A shared reference, in place of a Java object reference. Copies of a
// Ref point to one object, and the last copy to go deletes it.
template <class T> class Ref {
  struct Block { T* object; int refs; };
  Block* block = nullptr;
  void release() {
    if (block != nullptr && --block->refs == 0) {
      delete block->object;
      delete block;
    }
  }
public:
  Ref() = default;
  explicit Ref(T* object) : block(new Block{object, 1}) {}
  Ref(const Ref& other) : block(other.block) { if (block) block->refs++; }
  Ref& operator=(const Ref& other) {
    if (other.block) other.block->refs++;   // first, so self-assignment is safe
    release();
    block = other.block;
    return *this;
  }
  ~Ref() { release(); }
  T* operator->() const { return block->object; }   // OPR-5
  T& operator*() const { return *block->object; }
};

// Java's int arithmetic wraps, and C++ signed overflow is undefined
// (MUL-10, ADD-9). The hash runs in unsigned (MUL-8) and converts at the
// end (CONV-2).
inline int combine(unsigned h, int part) { return (int)(h * 31u + (unsigned)part); }

struct Point {
  int x, y;
  bool operator==(const Point&) const = default;    // DCMP-1
  int hash() const { return combine((unsigned)x, y); }
};

struct Segment {
  Point from, to;
  bool operator==(const Segment&) const = default;  // DCMP-1
  int hash() const { return combine((unsigned)from.hash(), to.hash()); }
  int lengthSquared() const {
    int dx = to.x - from.x, dy = to.y - from.y;
    return dx * dx + dy * dy;
  }
};

// A hash set, in place of java.util.HashSet. T needs hash() and ==.
template <class T> class HashSet {
  static const int bucketCount = 16;
  List<T> buckets[bucketCount];
  int count = 0;
public:
  HashSet(std::initializer_list<T> values) {           // ILC-3
    for (const T& v : values) add(v);
  }
  bool add(const T& value) {
    int r = value.hash() % bucketCount;                 // MUL-7: keeps the sign
    List<T>& chain = buckets[r < 0 ? r + bucketCount : r];
    for (const T& v : chain) if (v == value) return false;
    chain.add(value);
    count++;
    return true;
  }
  int size() const { return count; }
};

struct Account {
  const char* name;
  int balance;
  Account(const char* n, int b) : name(n), balance(b) {}
};

struct Book {
  List<Ref<Account>> accounts;
  void add(const Ref<Account>& a) { accounts.add(a); }
  int total() const {
    int t = 0;
    for (const Ref<Account>& a : accounts) t += a->balance;
    return t;
  }
};

int main() {
  int failures = 0;
  Segment a{{0, 0}, {3, 4}};
  Segment b{{0, 0}, {3, 4}};
  Segment c{{3, 4}, {0, 0}};
  if (!(a == b)) failures++;
  if (a == c) failures++;
  if (a.lengthSquared() != 25) failures++;
  HashSet<Segment> seen{a, b, c};
  if (seen.size() != 2) failures++;

  Ref<Account> shared(new Account("shared", 100));
  Book home, work;
  home.add(shared);
  home.add(Ref<Account>(new Account("home", 40)));
  work.add(shared);
  work.accounts.get(0)->balance += 25;
  if (home.total() != 165) failures++;
  if (work.total() != 125) failures++;
  return failures;
}
