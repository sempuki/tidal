// A sorted FlatMap<K, V, Cmp> over raw storage from the global allocation
// function: binary search with a comparator that can be transparent
// (lookup by const char* without building a key), insert_or_assign that
// shifts entries with moves, operator[], erase and erase_if, at() that
// throws, structured bindings over the entries, and a three-way
// comparison on the key class.
namespace std {
using size_t = decltype(sizeof 0);
class strong_ordering {
  signed char v;
public:
  constexpr explicit strong_ordering(signed char x) : v(x) {}
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

inline int live = 0;

class Str {
  char buf_[16] = {};
public:
  Str() { ++live; }
  Str(const char* s) { for (int i = 0; s[i] && i < 15; ++i) buf_[i] = s[i]; ++live; }
  Str(const Str& o) : Str(o.buf_) {}
  Str& operator=(const Str& o) { for (int i = 0; i < 16; ++i) buf_[i] = o.buf_[i]; return *this; }
  ~Str() { --live; }
  const char* c_str() const { return buf_; }
  friend std::strong_ordering operator<=>(const Str& a, const char* b) {
    const char* p = a.buf_;
    while (*p && *p == *b) { ++p; ++b; }
    unsigned char x = *p, y = *b;
    return x < y ? std::strong_ordering::less : x > y ? std::strong_ordering::greater : std::strong_ordering::equal;
  }
  friend std::strong_ordering operator<=>(const Str& a, const Str& b) { return a <=> b.buf_; }
  friend bool operator==(const Str& a, const Str& b) { return (a <=> b) == 0; }
  friend bool operator==(const Str& a, const char* b) { return (a <=> b) == 0; }
};

template <class T = void> struct Less {
  constexpr bool operator()(const T& a, const T& b) const { return a < b; }
};
template <> struct Less<void> {
  using is_transparent = void;
  template <class A, class B> constexpr bool operator()(const A& a, const B& b) const { return a < b; }
};

struct NotFound { const char* what; };

template <class K, class V, class Cmp = Less<K>> class FlatMap {
public:
  struct Entry { K key; V value; };
private:
  Entry* data_ = nullptr;
  std::size_t size_ = 0, cap_ = 0;
  [[no_unique_address]] Cmp cmp_{};

  template <class Q> static constexpr bool lookup_by = requires { typename Cmp::is_transparent; };

  template <class Q> std::size_t lower(const Q& k) const {
    std::size_t lo = 0, hi = size_;
    while (lo < hi) {
      std::size_t mid = lo + (hi - lo) / 2;
      if (cmp_(data_[mid].key, k)) lo = mid + 1; else hi = mid;
    }
    return lo;
  }
  void grow() {
    std::size_t cap = cap_ ? cap_ * 2 : 4;
    Entry* d = static_cast<Entry*>(::operator new(cap * sizeof(Entry)));
    for (std::size_t i = 0; i < size_; ++i) {
      ::new (&d[i]) Entry(move(data_[i]));
      data_[i].~Entry();
    }
    ::operator delete(data_);
    data_ = d;
    cap_ = cap;
  }
  template <class Q> bool same(std::size_t i, const Q& k) const {
    return i < size_ && !cmp_(k, data_[i].key);
  }

public:
  FlatMap() = default;
  FlatMap(const FlatMap&) = delete;
  FlatMap& operator=(const FlatMap&) = delete;
  ~FlatMap() {
    for (std::size_t i = 0; i < size_; ++i) data_[i].~Entry();
    ::operator delete(data_);
  }
  std::size_t size() const { return size_; }
  Entry* begin() { return data_; }
  Entry* end() { return data_ + size_; }
  const Entry* begin() const { return data_; }
  const Entry* end() const { return data_ + size_; }

  template <class Q> requires lookup_by<Q> || requires(const Q& q) { K(q); }
  V* find(const Q& k) {
    std::size_t i = lower(k);
    return same(i, k) ? &data_[i].value : nullptr;
  }
  template <class Q> bool contains(const Q& k) { return find(k) != nullptr; }

  template <class KK, class VV> bool insert_or_assign(KK&& k, VV&& v) {
    std::size_t i = lower(k);
    if (same(i, k)) { data_[i].value = forward<VV>(v); return false; }
    if (size_ == cap_) grow();
    if (i == size_) {
      ::new (&data_[size_]) Entry{K(forward<KK>(k)), V(forward<VV>(v))};
    } else {
      ::new (&data_[size_]) Entry(move(data_[size_ - 1]));
      for (std::size_t j = size_ - 1; j > i; --j) data_[j] = move(data_[j - 1]);
      data_[i] = Entry{K(forward<KK>(k)), V(forward<VV>(v))};
    }
    ++size_;
    return true;
  }
  V& operator[](const K& k) {
    std::size_t i = lower(k);
    if (!same(i, k)) { insert_or_assign(k, V{}); }
    return data_[i].value;
  }
  template <class Q> V& at(const Q& k) {
    if (V* p = find(k)) return *p;
    throw NotFound{"at"};
  }
  bool erase(const K& k) {
    std::size_t i = lower(k);
    if (!same(i, k)) return false;
    for (std::size_t j = i; j + 1 < size_; ++j) data_[j] = move(data_[j + 1]);
    data_[--size_].~Entry();
    return true;
  }
  template <class P> std::size_t erase_if(P pred) {
    std::size_t out = 0;
    for (std::size_t in = 0; in < size_; ++in) {
      if (!pred(data_[in])) {
        if (out != in) data_[out] = move(data_[in]);
        ++out;
      }
    }
    std::size_t removed = size_ - out;
    while (size_ > out) data_[--size_].~Entry();
    return removed;
  }
};

int main() {
  int fails = 0;
  {
    FlatMap<Str, int, Less<>> prices;
    prices.insert_or_assign("pear", 4);
    prices.insert_or_assign("apple", 3);
    prices.insert_or_assign("quince", 9);
    prices.insert_or_assign("fig", 7);
    prices.insert_or_assign("banana", 2);
    if (prices.insert_or_assign("apple", 5)) ++fails;
    if (prices.size() != 5) ++fails;
    const char* order[] = {"apple", "banana", "fig", "pear", "quince"};
    int i = 0;
    for (const auto& [k, v] : prices) { if (!(k == order[i])) ++fails; ++i; }
    if (!prices.contains("fig") || prices.contains("grape")) ++fails;
    if (*prices.find("apple") != 5) ++fails;
    prices[Str("grape")] += 6;
    if (prices.at("grape") != 6 || prices.size() != 6) ++fails;
    try {
      prices.at("kiwi");
      ++fails;
    } catch (const NotFound& e) {
      if (e.what[0] != 'a') ++fails;
    }
    std::size_t gone = prices.erase_if([](const auto& e) { return e.value < 5; });
    if (gone != 2 || prices.size() != 4) ++fails;
    if (!prices.erase(Str("fig")) || prices.erase(Str("fig"))) ++fails;
    int sum = 0;
    for (auto& [k, v] : prices) { v *= 2; sum += v; }
    if (sum != 2 * (5 + 4 + 9 + 6)) ++fails;
  }
  if (live != 0) ++fails;
  {
    FlatMap<int, Str> names;
    for (int k : {30, 10, 20}) names.insert_or_assign(k, "n");
    names[15] = "fifteen";
    int keys[] = {10, 15, 20, 30};
    int i = 0;
    for (const auto& e : names) if (e.key != keys[i++]) ++fails;
    if (!(names.at(15) == "fifteen")) ++fails;
  }
  if (live != 0) ++fails;
  return fails;
}
