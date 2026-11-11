// An open-addressing HashMap<K, V, H> with linear probing and tombstones,
// a Hash<T> primary template with specializations for unsigned int and for a
// string view, a transparent lookup by string literal, rehash that moves
// the entries into a new table, try_emplace that forwards a pack, erase,
// iteration that skips empty slots, and a move-only value type.
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

struct View {
  const char* p = nullptr;
  std::size_t n = 0;
  constexpr View() = default;
  constexpr View(const char* s) : p(s) { while (s[n]) ++n; }
  friend constexpr bool operator==(View a, View b) {
    if (a.n != b.n) return false;
    for (std::size_t i = 0; i < a.n; ++i) if (a.p[i] != b.p[i]) return false;
    return true;
  }
};

template <class T> struct Hash;
template <> struct Hash<unsigned> {
  constexpr std::size_t operator()(unsigned k) const noexcept { return k * 2654435761u; }
};
template <> struct Hash<View> {
  constexpr std::size_t operator()(View v) const noexcept {
    std::size_t h = 1469598103934665603u;
    for (std::size_t i = 0; i < v.n; ++i) { h ^= static_cast<unsigned char>(v.p[i]); h *= 1099511628211u; }
    return h;
  }
};

template <class K, class V, class H = Hash<K>> class HashMap {
  enum class State : unsigned char { empty, full, gone };
  // Second version: a named KV (the first try used an anonymous struct, CLS-9,
  // and ~decltype(kv), which DTOR-6 does not give).
  struct KV { K key; V value; };
  struct Slot {
    State state = State::empty;
    union { KV kv; };
    Slot() {}
    ~Slot() { if (state == State::full) kv.~KV(); }
  };
  Slot* slots_ = nullptr;
  std::size_t cap_ = 0, size_ = 0, used_ = 0;
  [[no_unique_address]] H hash_{};

  template <class Q> std::size_t probe(const Q& key) const {
    std::size_t i = hash_(key) & (cap_ - 1);
    std::size_t first_gone = cap_;
    while (slots_[i].state != State::empty) {
      if (slots_[i].state == State::full && slots_[i].kv.key == key) return i;
      if (slots_[i].state == State::gone && first_gone == cap_) first_gone = i;
      i = (i + 1) & (cap_ - 1);
    }
    return first_gone != cap_ ? first_gone : i;
  }
  void rehash(std::size_t ncap) {
    Slot* old = slots_;
    std::size_t ocap = cap_;
    // Third version: raw storage and one placement new per slot. The front
    // end reports NEW-5 for delete[] of a new[] array after a placement new
    // into an element's member.
    slots_ = static_cast<Slot*>(::operator new(ncap * sizeof(Slot)));
    for (std::size_t i = 0; i < ncap; ++i) ::new (static_cast<void*>(slots_ + i)) Slot();
    cap_ = ncap; size_ = 0; used_ = 0;
    for (std::size_t i = 0; i < ocap; ++i)
      if (old[i].state == State::full) try_emplace(move(old[i].kv.key), move(old[i].kv.value));
    release(old, ocap);
  }
  static void release(Slot* s, std::size_t n) {
    if (!s) return;
    for (std::size_t i = 0; i < n; ++i) s[i].~Slot();
    ::operator delete(s);
  }

public:
  struct Ref { const K& key; V& value; };
  HashMap() { rehash(8); }
  HashMap(const HashMap&) = delete;
  ~HashMap() { release(slots_, cap_); }
  std::size_t size() const { return size_; }

  template <class... A> bool try_emplace(K key, A&&... a) {
    if ((used_ + 1) * 4 > cap_ * 3) rehash(cap_ * 2);
    std::size_t i = probe(key);
    if (slots_[i].state == State::full) return false;
    if (slots_[i].state == State::empty) ++used_;
    ::new (static_cast<void*>(&slots_[i].kv)) KV{move(key), V(forward<A>(a)...)};
    slots_[i].state = State::full;
    ++size_;
    return true;
  }
  template <class Q> V* find(const Q& key) {
    std::size_t i = probe(key);
    return slots_[i].state == State::full ? &slots_[i].kv.value : nullptr;
  }
  V& operator[](const K& key) {
    if (V* v = find(key)) return *v;
    try_emplace(key);
    return *find(key);
  }
  template <class Q> bool erase(const Q& key) {
    std::size_t i = probe(key);
    if (slots_[i].state != State::full) return false;
    slots_[i].kv.~KV();
    slots_[i].state = State::gone;
    --size_;
    return true;
  }
  template <class F> void for_each(F f) {
    for (std::size_t i = 0; i < cap_; ++i)
      if (slots_[i].state == State::full) f(Ref{slots_[i].kv.key, slots_[i].kv.value});
  }
};

struct Counter {
  int* n;
  explicit Counter(int* n = nullptr) : n(n) {}
  Counter(Counter&& o) noexcept : n(o.n) { o.n = nullptr; }
  Counter& operator=(Counter&&) = delete;
  ~Counter() { if (n) ++*n; }
};

int main() {
  int fails = 0;
  {
    HashMap<unsigned, int> squares;
    for (unsigned i = 0; i < 40; ++i) squares[i] = static_cast<int>(i * i);
    if (squares.size() != 40 || *squares.find(7u) != 49) ++fails;
    for (unsigned i = 0; i < 40; i += 2) squares.erase(i);
    if (squares.size() != 20 || squares.find(8u) || !squares.find(9u)) ++fails;
    long total = 0;
    squares.for_each([&total](auto r) { total += r.value; });
    long expect = 0;
    for (unsigned i = 1; i < 40; i += 2) expect += i * i;
    if (total != expect) ++fails;
  }
  int destroyed = 0;
  {
    HashMap<View, Counter> names;
    names.try_emplace("alpha", &destroyed);
    names.try_emplace("beta", &destroyed);
    if (names.try_emplace("alpha", &destroyed)) ++fails;
    if (destroyed != 0) ++fails;
    if (!names.find(View("beta")) || names.find(View("gamma"))) ++fails;
    names.erase(View("alpha"));
    if (destroyed != 1 || names.size() != 1) ++fails;
    for (int i = 0; i < 12; ++i) {
      static const char* const words[] = {"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"};
      names.try_emplace(words[i]);
    }
    if (names.size() != 13 || destroyed != 1) ++fails;
  }
  if (destroyed != 2) ++fails;
  return fails;
}
