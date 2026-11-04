// A fixed-capacity open-addressing hash map in raw storage, with a concept
// for keys, dependent names, and an explicit instantiation.
using size_t = decltype(sizeof 0);
inline void* operator new(size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}

constexpr size_t hash(int v) { return size_t(v) * 2654435761u; }

template <class K>
concept Hashable = requires(const K& k) {
  hash(k);
  k == k;
};

namespace geo {
struct Point { int x, y; bool operator==(const Point&) const = default; };
constexpr size_t hash(const Point& p) { return size_t(p.x) * 31u + size_t(p.y); }
}

template <class V> struct Traits { using stored = V; static constexpr bool small = sizeof(V) <= 8; };

template <Hashable K, class V, size_t N>
class Map {
public:
  using value_type = typename Traits<V>::stored;
  struct Slot {
    bool used;
    alignas(K) unsigned char key[sizeof(K)];
    alignas(value_type) unsigned char value[sizeof(value_type)];
    K* k() { return reinterpret_cast<K*>(key); }
    value_type* v() { return reinterpret_cast<value_type*>(value); }
  };
private:
  Slot slots[N] = {};
  size_t count = 0;
  template <class F> void each(F f) { for (Slot& s : slots) if (s.used) f(s); }
public:
  Map() = default;
  Map(const Map&) = delete;
  ~Map() {
    this->each([](Slot& s) { s.k()->~K(); s.v()->~value_type(); });
  }
  bool put(const K& k, const value_type& v);
  value_type* find(const K& k);
  size_t size() const { return count; }
};

template <Hashable K, class V, size_t N>
bool Map<K, V, N>::put(const K& k, const value_type& v) {
  size_t i = hash(k) % N;
  for (size_t step = 0; step < N; ++step, i = (i + 1) % N) {
    Slot& s = slots[i];
    if (s.used && *s.k() == k) { *s.v() = v; return true; }
    if (!s.used) {
      ::new (static_cast<void*>(s.key)) K(k);
      ::new (static_cast<void*>(s.value)) value_type(v);
      s.used = true;
      ++count;
      return true;
    }
  }
  return false;
}

template <Hashable K, class V, size_t N>
typename Map<K, V, N>::value_type* Map<K, V, N>::find(const K& k) {
  size_t i = hash(k) % N;
  for (size_t step = 0; step < N; ++step, i = (i + 1) % N) {
    Slot& s = slots[i];
    if (!s.used) return nullptr;
    if (*s.k() == k) return s.v();
  }
  return nullptr;
}

template class Map<int, long, 8>;   // explicit instantiation definition

struct Name {
  const char* text;
  int length;
  Name(const char* t) : text(t), length(0) { while (t[length]) ++length; }
  Name(const Name&) = default;
  ~Name() { text = nullptr; }
};

int main() {
  Map<int, long, 8> squares;
  for (int i = 0; i < 6; ++i) squares.put(i, long(i) * i);
  if (squares.size() != 6) return 1;
  if (*squares.find(5) != 25) return 2;
  if (squares.find(7) != nullptr) return 3;

  Map<geo::Point, Name, 4> names;
  names.put({1, 2}, "first");
  names.put({3, 4}, "second");
  names.put({1, 2}, "again");
  if (names.size() != 2) return 4;
  if (names.find({1, 2})->length != 5) return 5;
  if (names.put({5, 6}, "x") != true || names.put({7, 8}, "y") != true) return 6;
  if (names.put({9, 9}, "z")) return 7;   // full
  return 0;
}
