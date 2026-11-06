// StaticVector<T, N>: fixed capacity, elements in raw storage inside the object.
// Rules used: LSUP-7 (placement form), STG-4 and STG-P2 (rows host objects),
// STG-5 and STG-29 (keep the pointer that placement new returns),
// DTOR-6 (explicit destructor call), CPY-P2 and CPY-8 (own copy and move),
// PACK-22 and PACK-23 (forwarding with static_cast<Args&&>), TPL-13.

namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* at) noexcept { return at; }

template <class T, int N>
class StaticVector {
  alignas(T) unsigned char rows[N][sizeof(T)];
  T* items[N];
  int count = 0;
public:
  StaticVector() = default;
  StaticVector(const StaticVector& o) {
    for (int i = 0; i < o.count; ++i) emplace_back(*o.items[i]);
  }
  StaticVector(StaticVector&& o) {
    for (int i = 0; i < o.count; ++i) emplace_back(static_cast<T&&>(*o.items[i]));
    o.clear();
  }
  StaticVector& operator=(const StaticVector& o) {
    if (this != &o) {
      clear();
      for (int i = 0; i < o.count; ++i) emplace_back(*o.items[i]);
    }
    return *this;
  }
  StaticVector& operator=(StaticVector&& o) {
    if (this != &o) {
      clear();
      for (int i = 0; i < o.count; ++i) emplace_back(static_cast<T&&>(*o.items[i]));
      o.clear();
    }
    return *this;
  }
  ~StaticVector() { clear(); }

  template <class... Args>
  T& emplace_back(Args&&... args) {
    T* p = new (rows[count]) T(static_cast<Args&&>(args)...);
    items[count++] = p;
    return *p;
  }
  void pop_back() { items[--count]->~T(); }
  void clear() { while (count > 0) pop_back(); }
  int size() const { return count; }
  static constexpr int capacity() { return N; }
  T& operator[](int i) { return *items[i]; }
  const T& operator[](int i) const { return *items[i]; }
};

struct Counter {
  static int constructed;
  static int destroyed;
  int a, b;
  Counter(int x, int y) : a(x), b(y) { ++constructed; }
  Counter(const Counter& o) : a(o.a), b(o.b) { ++constructed; }
  Counter(Counter&& o) : a(o.a), b(o.b) { o.a = -1; ++constructed; }
  ~Counter() { ++destroyed; }
};
int Counter::constructed = 0;
int Counter::destroyed = 0;

static_assert(StaticVector<int, 3>::capacity() == 3);

int test() {
  {
    StaticVector<Counter, 4> v;
    v.emplace_back(1, 2);
    v.emplace_back(3, 4);
    v.emplace_back(5, 6);
    if (v.size() != 3) return 1;
    if (Counter::constructed != 3) return 2;
    v.pop_back();
    if (Counter::destroyed != 1) return 3;
    StaticVector<Counter, 4> c = v;               // copy
    if (Counter::constructed != 5 || c[1].b != 4) return 4;
    StaticVector<Counter, 4> m = static_cast<StaticVector<Counter, 4>&&>(c);  // move
    if (m.size() != 2 || c.size() != 0 || m[0].a != 1) return 5;
    v = m;                                         // copy assignment
    v = static_cast<StaticVector<Counter, 4>&&>(m); // move assignment
    if (v.size() != 2 || v[1].a != 3) return 6;
  }
  if (Counter::constructed != Counter::destroyed) return 7;
  return 0;
}

int main() { return test(); }
