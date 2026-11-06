// A small string with inline storage and a view type: a converting
// constructor from a string literal, a conversion function from String to
// View, hidden-friend operator+ and == that take const references and get
// their operands through those conversions, a user-defined literal, a fold
// over + that joins a pack, the conditional operator with a String and a
// string literal, an insertion sort with a comparator, and a function that
// takes const View& and is called with a String.
namespace std { using size_t = decltype(sizeof 0); }

class View {
  const char* p_ = "";
  std::size_t n_ = 0;
public:
  constexpr View() = default;
  constexpr View(const char* p, std::size_t n) : p_(p), n_(n) {}
  constexpr std::size_t size() const { return n_; }
  constexpr char operator[](std::size_t i) const { return p_[i]; }
  constexpr const char* begin() const { return p_; }
  constexpr const char* end() const { return p_ + n_; }
  friend constexpr bool operator==(View a, View b) {
    if (a.n_ != b.n_) return false;
    for (std::size_t i = 0; i < a.n_; ++i)
      if (a.p_[i] != b.p_[i]) return false;
    return true;
  }
  friend constexpr bool operator<(View a, View b) {
    for (std::size_t i = 0; i < a.n_ && i < b.n_; ++i)
      if (a.p_[i] != b.p_[i]) return a.p_[i] < b.p_[i];
    return a.n_ < b.n_;
  }
};

class String {
  static constexpr std::size_t cap = 31;
  char buf_[cap + 1] = {};
  std::size_t len_ = 0;
public:
  constexpr String() = default;
  constexpr String(const char* s) {
    while (s[len_] && len_ < cap) { buf_[len_] = s[len_]; ++len_; }
  }
  constexpr explicit String(View v) {
    for (char c : v) if (len_ < cap) buf_[len_++] = c;
  }
  constexpr operator View() const { return View(buf_, len_); }
  constexpr std::size_t size() const { return len_; }
  constexpr String& operator+=(View v) {
    for (char c : v) if (len_ < cap) buf_[len_++] = c;
    return *this;
  }
  friend constexpr String operator+(const String& a, const String& b) {
    String r = a;
    r += b;
    return r;
  }
  friend constexpr bool operator==(const String& a, const String& b) {
    View va = a, vb = b;           // copy-initialization: the front end runs it
    return va == vb;
  }
};

constexpr String operator""_s(const char* s, std::size_t n) { return String(View(s, n)); }

template <class... Parts>
constexpr String join(const Parts&... parts) {
  return (String() + ... + parts);
}

constexpr std::size_t count_char(const View& v, char c) {
  std::size_t n = 0;
  for (char x : v) n += x == c;
  return n;
}

template <class T, std::size_t N, class Less>
constexpr void insertion_sort(T (&a)[N], Less less) {
  for (std::size_t i = 1; i < N; ++i)
    for (std::size_t j = i; j > 0 && less(a[j], a[j - 1]); --j) {
      T t = a[j];
      a[j] = a[j - 1];
      a[j - 1] = t;
    }
}

constexpr int run(bool loud) {
  String hello = "hello";
  String all = hello + ", " + "world"_s;           // const char* binds to const String&
  if (!(all == "hello, world")) return 1;
  if (count_char(all, 'o') != 2) return 2;          // String binds to const View&
  String j = join("a"_s, String("b"), "c");
  if (!(j == "abc")) return 3;
  String greeting = loud ? hello : "quiet";         // class lvalue against a literal
  if (greeting.size() != (loud ? 5u : 5u)) return 4;
  String words[4] = {"pear", "fig", "apple", "kiwi"};
  insertion_sort(words, [](const String& a, const String& b) { View va = a, vb = b; return va < vb; });
  if (!(words[0] == "apple") || !(words[3] == "pear")) return 5;
  return 0;
}

static_assert(run(true) == 0);
static_assert(run(false) == 0);

int main(int argc, char**) { return run(argc > 1); }
