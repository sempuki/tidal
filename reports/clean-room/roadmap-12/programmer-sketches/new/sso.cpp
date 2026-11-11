// Str: a string with a small buffer in the object and a free-store
// buffer for longer text, chosen by a union with an active member that
// changes on growth. A StrView with constexpr find and starts_with, a
// literal operator _sv, append and +, <=> and == against views and
// C strings, a consteval hash, and a switch over hashed command names.
namespace std {
using size_t = decltype(sizeof 0);
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, greater;
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
using std::size_t;

constexpr size_t length(const char* s) {
  size_t n = 0;
  while (s[n]) ++n;
  return n;
}

class StrView {
  const char* p_ = "";
  size_t n_ = 0;

 public:
  static constexpr size_t npos = static_cast<size_t>(-1);
  constexpr StrView() = default;
  constexpr StrView(const char* s) : p_(s), n_(length(s)) {}
  constexpr StrView(const char* s, size_t n) : p_(s), n_(n) {}
  constexpr const char* data() const { return p_; }
  constexpr size_t size() const { return n_; }
  constexpr char operator[](size_t i) const { return p_[i]; }
  constexpr const char* begin() const { return p_; }
  constexpr const char* end() const { return p_ + n_; }
  constexpr StrView substr(size_t pos, size_t n = npos) const {
    if (pos > n_) pos = n_;
    if (n > n_ - pos) n = n_ - pos;
    return {p_ + pos, n};
  }
  constexpr bool starts_with(StrView o) const { return o.n_ <= n_ && substr(0, o.n_) == o; }
  constexpr size_t find(StrView o, size_t from = 0) const {
    for (size_t i = from; i + o.n_ <= n_; ++i)
      if (substr(i, o.n_) == o) return i;
    return npos;
  }
  friend constexpr bool operator==(StrView a, StrView b) {
    if (a.n_ != b.n_) return false;
    for (size_t i = 0; i < a.n_; ++i)
      if (a.p_[i] != b.p_[i]) return false;
    return true;
  }
  friend constexpr std::strong_ordering operator<=>(StrView a, StrView b) {
    size_t m = a.n_ < b.n_ ? a.n_ : b.n_;
    for (size_t i = 0; i < m; ++i)
      if (a.p_[i] != b.p_[i])
        return static_cast<unsigned char>(a.p_[i]) < static_cast<unsigned char>(b.p_[i])
                   ? std::strong_ordering::less : std::strong_ordering::greater;
    return a.n_ < b.n_ ? std::strong_ordering::less
         : a.n_ > b.n_ ? std::strong_ordering::greater : std::strong_ordering::equal;
  }
};

constexpr StrView operator""_sv(const char* s, size_t n) { return {s, n}; }

consteval unsigned hash(StrView s) {
  unsigned h = 2166136261u;
  for (char c : s) h = (h ^ static_cast<unsigned char>(c)) * 16777619u;
  return h;
}
constexpr unsigned hash_rt(StrView s) {
  unsigned h = 2166136261u;
  for (char c : s) h = (h ^ static_cast<unsigned char>(c)) * 16777619u;
  return h;
}

class Str {
  static constexpr size_t small_cap = 15;
  size_t size_ = 0;
  bool heap_ = false;
  // Second try: the heap part is a named member type, since an anonymous
  // union cannot declare a type (UNI-9).
  struct Heap {
    char* ptr;
    size_t cap;
  };
  union {
    char small_[small_cap + 1];
    Heap big_;
  };
  constexpr void steal(Str& o) {
    size_ = o.size_;
    heap_ = o.heap_;
    if (heap_) {
      big_ = o.big_;
      o.heap_ = false;
      o.size_ = 0;
      o.small_[0] = 0;
    } else {
      for (size_t i = 0; i <= size_; ++i) small_[i] = o.small_[i];
    }
  }

  constexpr char* buf() { return heap_ ? big_.ptr : small_; }
  constexpr const char* buf() const { return heap_ ? big_.ptr : small_; }
  constexpr size_t cap() const { return heap_ ? big_.cap : small_cap; }

 public:
  constexpr Str() : small_{} {}
  constexpr Str(StrView v) : Str() { append(v); }
  constexpr Str(const char* s) : Str(StrView(s)) {}
  constexpr Str(const Str& o) : Str() { append(o); }
  constexpr Str(Str&& o) noexcept : small_{} { steal(o); }
  // Second try: no destroy and placement new, which needs a declared
  // placement form and is not a constant expression; steal in place.
  constexpr Str& operator=(Str o) noexcept {
    if (heap_) delete[] big_.ptr;
    heap_ = false;
    small_[0] = 0;
    steal(o);
    return *this;
  }
  constexpr ~Str() {
    if (heap_) delete[] big_.ptr;
  }

  constexpr size_t size() const { return size_; }
  constexpr bool is_small() const { return !heap_; }
  constexpr const char* c_str() const { return buf(); }
  constexpr operator StrView() const { return {buf(), size_}; }

  constexpr void reserve(size_t want) {
    if (want <= cap()) return;
    size_t nc = cap() * 2 > want ? cap() * 2 : want;
    char* p = new char[nc + 1];
    for (size_t i = 0; i <= size_; ++i) p[i] = buf()[i];
    if (heap_) delete[] big_.ptr;
    big_.ptr = p;
    big_.cap = nc;
    heap_ = true;
  }
  constexpr Str& append(StrView v) {
    reserve(size_ + v.size());
    for (size_t i = 0; i < v.size(); ++i) buf()[size_ + i] = v[i];
    size_ += v.size();
    buf()[size_] = 0;
    return *this;
  }
  constexpr Str& operator+=(StrView v) { return append(v); }
  friend constexpr Str operator+(Str a, StrView b) { return a += b; }
};

constexpr bool check_constant() {
  Str s = "hello";
  if (!s.is_small()) return false;
  s += ", world, and some more text";
  if (s.is_small() || s.size() != 32) return false;
  StrView v = s;
  if (v.find("world") != 7 || !v.starts_with("hello"_sv)) return false;
  Str t = s;
  if (!(StrView(t) == v)) return false;
  Str u = static_cast<Str&&>(t);
  if (t.size() != 0 || u.size() != 32) return false;
  return StrView("apple") < StrView("banana") && "b"_sv > "abc"_sv;
}
static_assert(check_constant());
static_assert(hash("go") != hash("stop"));

int run(StrView cmd) {
  switch (hash_rt(cmd)) {
    case hash("go"): return 1;
    case hash("stop"): return 2;
    case hash("wait"): return 3;
    default: return 0;
  }
}

int main() {
  int fails = 0;
  if (!check_constant()) ++fails;
  Str a = "short";
  Str b = a + " and now a longer tail";
  if (!a.is_small() || b.is_small() || b.size() != 27) ++fails;
  if (StrView(b).find("tail") != 23) ++fails;
  a = b;
  if (!(StrView(a) == StrView(b)) || a.is_small()) ++fails;
  b = "x";
  if (StrView(b) != "x"_sv) ++fails;
  if (run("go") + run("stop") + run("wait") + run("other") != 6) ++fails;
  Str parts[3] = {"one", "two", "three"};
  Str joined;
  for (const Str& p : parts) { if (joined.size()) joined += ","; joined += p; }
  if (StrView(joined) != "one,two,three") ++fails;
  return fails;
}
