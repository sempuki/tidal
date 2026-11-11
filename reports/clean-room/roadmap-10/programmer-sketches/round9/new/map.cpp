// A sorted flat map: try_emplace that forwards a pack into the value's
// constructor through an Entry constructor (a pack in a member initializer),
// a heterogeneous find that compares a Key with a string literal through
// hidden friends, operator[] that default-constructs, structured bindings in
// a range for, a class key with defaulted <=>-free ordering written by hand,
// and a constrained callback that visits each entry.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept {
  return static_cast<T&&>(t);
}

class Key {
  char s_[12] = {};
public:
  constexpr Key() = default;
  constexpr Key(const char* p) {
    for (int i = 0; i < 11 && p[i]; ++i) s_[i] = p[i];
  }
  friend constexpr int compare(const Key& a, const char* b) {
    int i = 0;
    for (; a.s_[i] && a.s_[i] == b[i]; ++i) {}
    return a.s_[i] - b[i];
  }
  friend constexpr bool operator<(const Key& a, const Key& b) { return compare(a, b.s_) < 0; }
  friend constexpr bool operator==(const Key& a, const char* b) { return compare(a, b) == 0; }
};

template <class K, class V, std::size_t N>
class FlatMap {
public:
  struct Entry {
    K key;
    V value;
    Entry() = default;
    template <class KK, class... A>
    constexpr Entry(KK&& k, A&&... a) : key(forward<KK>(k)) { value = V(forward<A>(a)...); }
  };
private:
  Entry items_[N];
  std::size_t n_ = 0;

  template <class Q>
  constexpr std::size_t lower(const Q& q) const {
    std::size_t lo = 0, hi = n_;
    while (lo < hi) {
      std::size_t mid = (lo + hi) / 2;
      if (items_[mid].key < q) lo = mid + 1; else hi = mid;
    }
    return lo;
  }
public:
  template <class KK, class... A>
  constexpr bool try_emplace(KK&& k, A&&... a) {
    K key(forward<KK>(k));
    std::size_t i = lower(key);
    if (i < n_ && !(key < items_[i].key)) return false;
    for (std::size_t j = n_; j > i; --j) items_[j] = move(items_[j - 1]);
    items_[i] = Entry(move(key), forward<A>(a)...);
    ++n_;
    return true;
  }
  constexpr V* find(const char* k) {
    for (std::size_t i = 0; i < n_; ++i)
      if (items_[i].key == k) return &items_[i].value;
    return nullptr;
  }
  constexpr V& operator[](const K& k) {
    try_emplace(k);
    return *find_key(k);
  }
  constexpr V* find_key(const K& k) {
    std::size_t i = lower(k);
    return i < n_ && !(k < items_[i].key) ? &items_[i].value : nullptr;
  }
  constexpr std::size_t size() const { return n_; }
  constexpr const Entry* begin() const { return items_; }
  constexpr const Entry* end() const { return items_ + n_; }

  template <class F>
    requires requires(F f, const K& k, const V& v) { f(k, v); }
  constexpr void each(F f) const {
    for (const auto& [k, v] : *this) f(k, v);
  }
};

struct Range {
  int lo = 0, hi = 0;
  constexpr Range() = default;
  constexpr Range(int a, int b) : lo(a), hi(b) {}
};

constexpr int run() {
  FlatMap<Key, Range, 8> m;
  if (!m.try_emplace("pear", 3, 9)) return 1;      // two ints into Range(int, int)
  if (!m.try_emplace("apple", 1, 2)) return 2;
  if (m.try_emplace("pear", 0, 0)) return 3;       // already there
  m["fig"] = Range(4, 4);                           // default-constructs, then assigns
  if (m.size() != 3) return 4;
  Range* r = m.find("apple");
  if (!r || r->hi != 2) return 5;
  int width = 0;
  m.each([&width](const Key&, const Range& v) { width += v.hi - v.lo; });
  if (width != 1 + 0 + 6) return 6;
  const auto& first = *m.begin();
  return first.key == "apple" ? 0 : 7;              // kept in order
}

static_assert(run() == 0);
int main() { return run(); }
