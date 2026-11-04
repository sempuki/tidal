// hashmap.cpp: an open-addressing hash map in raw storage.
// Written from the Tidal specification alone. No standard header.
//
// Rules that decided each construct:
//   std::size_t declared by the program ............ LSUP-4
//   non-allocating placement forms defined here ..... LSUP-7, STG-5
//   byte array hosts the entries .................... STG-4, STG-P2
//   keep the pointer that placement new returns ..... STG-29, STG-P4
//   alignas on the byte array ....................... ALN-1, STG-17
//   explicit destructor call ends the entry ......... DTOR-6, STG-6
//   own copy functions (deleted) .................... CPY-8 (storage page example)
//   concept, requires expression, compound req ...... CPT-P2, CPT-P3, CPT-33
//   requires clause operands in parentheses ......... CPT-P4, CPT-10
//   members defined after the class body ............ TPL-23, TPL-43, CNS-26
//   member template defined outside: two heads ...... TPL-24
//   qualified return type without typename .......... DEP-20

namespace std {
using size_t = decltype(sizeof 0);
}

inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}

// A same-type test. The specification uses `same<T, U>` in an example
// (declarations/auto.md) without a declaration, so the program writes
// its own with a partial specialization of a variable template.
template <class T, class U> constexpr bool same_type = false;
template <class T> constexpr bool same_type<T, T> = true;
template <class T, class U> concept SameAs = same_type<T, U>;

// Hash functions for the built-in key types. They are declared before
// the concept, because a fundamental type has no associated namespace.
inline std::size_t hash_of(int k) {
  unsigned long long x = static_cast<unsigned long long>(static_cast<long long>(k));
  return static_cast<std::size_t>(x);  // identity: tests choose collisions
}
inline std::size_t hash_of(unsigned k) { return static_cast<std::size_t>(k) * 3u; }

// A key type needs a hash of type std::size_t and an equality of type bool.
template <class K>
concept Key = requires (const K& a, const K& b) {
  { hash_of(a) } -> SameAs<std::size_t>;
  { a == b } -> SameAs<bool>;
};

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
class HashMap {
public:
  struct Entry {
    K key;
    V value;
  };

  HashMap() = default;
  HashMap(const HashMap&) = delete;             // the slots hold pointers into
  HashMap& operator=(const HashMap&) = delete;  // this object's own bytes
  ~HashMap();

  bool insert(const K& key, const V& value);    // false when the map is full
  V* find(const K& key);
  const V* find(const K& key) const;
  bool erase(const K& key);
  void clear();
  std::size_t size() const;
  bool home_check() const;                      // every entry is reachable

  template <class F> void for_each(F visit) const;

private:
  struct Slot {
    alignas(Entry) unsigned char bytes[sizeof(Entry)];
    Entry* held = nullptr;                      // null, or the placement-new result
  };

  static constexpr std::size_t mask = Capacity - 1;
  static constexpr std::size_t none = Capacity;

  Slot slots[Capacity];
  std::size_t count = 0;

  static std::size_t home(const K& key);
  static std::size_t next(std::size_t i);
  static std::size_t distance(std::size_t from, std::size_t to);
  std::size_t locate(const K& key) const;
  void build(std::size_t at, Entry&& from);
  void destroy(std::size_t at);
};

// ---- members, defined after the class body (TPL-23) ----

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
HashMap<K, V, Capacity>::~HashMap() {
  clear();
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
std::size_t HashMap<K, V, Capacity>::home(const K& key) {
  return hash_of(key) & mask;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
std::size_t HashMap<K, V, Capacity>::next(std::size_t i) {
  return (i + 1) & mask;
}

// How many steps forward from `from` reach `to`, going round the table.
template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
std::size_t HashMap<K, V, Capacity>::distance(std::size_t from, std::size_t to) {
  return (to - from) & mask;                    // unsigned: wraps (MUL-P2)
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
std::size_t HashMap<K, V, Capacity>::locate(const K& key) const {
  std::size_t i = home(key);
  for (std::size_t step = 0; step != Capacity; ++step) {
    const Slot& s = slots[i];
    if (s.held == nullptr) return none;         // an empty slot ends the run
    if (s.held->key == key) return i;
    i = next(i);
  }
  return none;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
void HashMap<K, V, Capacity>::build(std::size_t at, Entry&& from) {
  // Move the entry into the bytes of slot `at` and keep the pointer.
  slots[at].held = new (slots[at].bytes) Entry(static_cast<Entry&&>(from));
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
void HashMap<K, V, Capacity>::destroy(std::size_t at) {
  slots[at].held->~Entry();                     // ends the entry, keeps the bytes
  slots[at].held = nullptr;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
bool HashMap<K, V, Capacity>::insert(const K& key, const V& value) {
  std::size_t found = locate(key);
  if (found != none) {
    slots[found].held->value = value;
    return true;
  }
  if (count == Capacity) return false;
  std::size_t i = home(key);
  while (slots[i].held != nullptr) i = next(i);
  slots[i].held = new (slots[i].bytes) Entry{key, value};
  ++count;
  return true;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
V* HashMap<K, V, Capacity>::find(const K& key) {
  std::size_t i = locate(key);
  return i == none ? nullptr : &slots[i].held->value;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
const V* HashMap<K, V, Capacity>::find(const K& key) const {
  std::size_t i = locate(key);
  return i == none ? nullptr : &slots[i].held->value;
}

// Erase by backward shift. After the hole at `gap`, each entry of the
// run moves back into the hole if its home is not between the hole and
// its own slot. Then no empty slot breaks a later search.
template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
bool HashMap<K, V, Capacity>::erase(const K& key) {
  std::size_t gap = locate(key);
  if (gap == none) return false;
  destroy(gap);
  --count;
  std::size_t j = next(gap);
  while (slots[j].held != nullptr) {
    std::size_t h = home(slots[j].held->key);
    if (distance(h, j) >= distance(gap, j)) {
      build(gap, static_cast<Entry&&>(*slots[j].held));
      destroy(j);
      gap = j;
    }
    j = next(j);
  }
  return true;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
void HashMap<K, V, Capacity>::clear() {
  for (std::size_t i = 0; i != Capacity; ++i)
    if (slots[i].held != nullptr) destroy(i);
  count = 0;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
std::size_t HashMap<K, V, Capacity>::size() const {
  return count;
}

template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
bool HashMap<K, V, Capacity>::home_check() const {
  std::size_t seen = 0;
  for (std::size_t i = 0; i != Capacity; ++i) {
    if (slots[i].held == nullptr) continue;
    ++seen;
    if (locate(slots[i].held->key) != i) return false;
  }
  return seen == count;
}

// A member template of a class template: the class head, then its own.
template <Key K, class V, std::size_t Capacity>
  requires (Capacity > 0 && (Capacity & (Capacity - 1)) == 0)
template <class F>
void HashMap<K, V, Capacity>::for_each(F visit) const {
  for (std::size_t i = 0; i != Capacity; ++i)
    if (slots[i].held != nullptr) visit(slots[i].held->key, slots[i].held->value);
}

// ---- test types ----

// A value that counts its live objects, to check every destructor runs.
struct Tracked {
  static inline int live = 0;
  int value;
  Tracked(int v) : value(v) { ++live; }
  Tracked(const Tracked& o) : value(o.value) { ++live; }
  Tracked(Tracked&& o) noexcept : value(o.value) { ++live; }
  Tracked& operator=(const Tracked& o) { value = o.value; return *this; }
  ~Tracked() { --live; }
};

// A class key. hash_of is a friend, found by argument-dependent lookup.
struct Name {
  char text[8];
  friend bool operator==(const Name& a, const Name& b) {
    for (int i = 0; i != 8; ++i)
      if (a.text[i] != b.text[i]) return false;
    return true;
  }
  friend std::size_t hash_of(const Name& n) {
    std::size_t h = 1469598103u;
    for (int i = 0; i != 8 && n.text[i] != 0; ++i)
      h = (h ^ static_cast<unsigned char>(n.text[i])) * 16777619u;
    return h;
  }
};

struct NoHash { int v; };
struct BadHash { int v; };
inline int hash_of(const BadHash& b) { return b.v; }   // wrong result type
inline bool operator==(const BadHash&, const BadHash&) { return true; }

static_assert(Key<int>);
static_assert(Key<Name>);
static_assert(!Key<NoHash>);
static_assert(!Key<BadHash>);
static_assert(alignof(HashMap<int, double, 8>::Entry) == alignof(double));

// ---- checks ----

int failures = 0;
void check(bool ok) { if (!ok) ++failures; }

void collisions() {
  HashMap<int, Tracked, 16> map;
  // 1, 17, 33, 49 share home 1. 15, 31 share home 15 and wrap to 0, 1...
  int keys[] = {1, 17, 33, 49, 15, 31, 47, 2};
  for (int k : keys) check(map.insert(k, Tracked(k * 10)));
  check(map.size() == 8);
  check(Tracked::live == 8);
  for (int k : keys) {
    Tracked* t = map.find(k);
    check(t != nullptr && t->value == k * 10);
  }
  check(map.erase(17));                 // the middle of the run at home 1
  check(!map.erase(17));
  check(map.find(17) == nullptr);
  check(map.find(33) != nullptr && map.find(33)->value == 330);
  check(map.find(49) != nullptr && map.find(49)->value == 490);
  check(map.erase(15));                 // the start of the wrapped run
  check(map.find(31) != nullptr && map.find(31)->value == 310);
  check(map.find(47) != nullptr && map.find(47)->value == 470);
  check(map.find(2) != nullptr && map.find(2)->value == 20);
  check(map.home_check());
  check(map.size() == 6);
  check(Tracked::live == 6);
  check(map.insert(33, Tracked(7)));    // replaces the value
  check(map.find(33)->value == 7);
  check(map.size() == 6);
  int sum = 0;
  map.for_each([&sum](const int& k, const Tracked& v) { sum += k + v.value; });
  check(sum == (1 + 33 + 49 + 31 + 47 + 2) + (10 + 7 + 490 + 310 + 470 + 20));
}

void fill_and_empty() {
  HashMap<unsigned, int, 8> map;
  for (unsigned k = 0; k != 8; ++k) check(map.insert(k * 8, static_cast<int>(k)));
  check(!map.insert(99, 0));            // full
  check(map.home_check());
  for (unsigned k = 0; k != 8; k += 2) check(map.erase(k * 8));
  check(map.home_check());
  for (unsigned k = 1; k < 8; k += 2) {
    const HashMap<unsigned, int, 8>& view = map;
    const int* v = view.find(k * 8);
    check(v != nullptr && *v == static_cast<int>(k));
  }
  for (unsigned k = 0; k < 8; k += 2) check(map.find(k * 8) == nullptr);
  check(map.size() == 4);
}

void names() {
  HashMap<Name, int, 32> map;
  Name a{"ada"}, b{"pascal"}, c{"lisp"}, d{"algol"};
  check(map.insert(a, 1983));
  check(map.insert(b, 1970));
  check(map.insert(c, 1958));
  check(map.find(d) == nullptr);
  check(map.erase(b));
  check(map.find(a) != nullptr && *map.find(a) == 1983);
  check(map.find(c) != nullptr && *map.find(c) == 1958);
  check(map.home_check());
}

// Many inserts and erases against a plain array as the model.
void churn() {
  HashMap<int, int, 64> map;
  int model[200];
  bool present[200];
  for (int i = 0; i != 200; ++i) { model[i] = 0; present[i] = false; }
  unsigned state = 12345u;
  for (int round = 0; round != 5000; ++round) {
    state = state * 1103515245u + 12345u;     // unsigned wraps (MUL-P2)
    int k = static_cast<int>((state >> 8) % 200u);
    bool add = ((state >> 20) & 1u) != 0u;
    if (add) {
      if (present[k] || map.size() < 48) {
        check(map.insert(k * 64, round));     // all keys share home 0
        model[k] = round;
        present[k] = true;
      }
    } else {
      check(map.erase(k * 64) == present[k]);
      present[k] = false;
    }
  }
  int n = 0;
  for (int i = 0; i != 200; ++i) {
    const int* v = map.find(i * 64);
    if (present[i]) { ++n; check(v != nullptr && *v == model[i]); }
    else check(v == nullptr);
  }
  check(map.size() == static_cast<std::size_t>(n));
  check(map.home_check());
}

int main() {
  collisions();
  check(Tracked::live == 0);            // the destructor ended every entry
  fill_and_empty();
  names();
  churn();
  return failures == 0 ? 0 : 1;
}
