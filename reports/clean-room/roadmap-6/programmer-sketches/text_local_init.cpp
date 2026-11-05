// text.cpp: a small string with a short buffer, words, and a pair with
// the tuple protocol. No headers. Round 6 sketch, programmer view.
namespace std {
using size_t = decltype(sizeof 0);
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}  // namespace std

inline void* operator new(std::size_t, void* p) noexcept { return p; }

// A string that keeps up to 15 characters inside the object.
class Str {
  static constexpr std::size_t short_cap = 15;
  std::size_t len = 0;
  std::size_t cap = short_cap;
  union {
    char local[short_cap + 1];
    char* heap;
  };
  bool is_short() const { return cap == short_cap; }
  char* buf() { return is_short() ? local : heap; }

public:
  Str() : local{} {}
  Str(const char* s) : Str() {
    while (*s) push(*s++);
  }
  Str(const Str& o) : Str() {
    for (char c : o) push(c);
  }
  Str(Str&& o) noexcept : len(o.len), cap(o.cap), local{} {
    if (o.is_short()) {
      for (std::size_t i = 0; i <= short_cap; ++i) local[i] = o.local[i];
    } else {
      heap = o.heap;
      o.cap = short_cap;
      o.local[0] = '\0';
    }
    o.len = 0;
  }
  Str& operator=(Str o) noexcept {
    this->~Str();
    ::new (this) Str(static_cast<Str&&>(o));
    return *this;
  }
  ~Str() {
    if (!is_short()) delete[] heap;
  }

  void push(char c) {
    if (len == cap) {
      std::size_t next = cap * 2 + 1;
      char* fresh = new char[next + 1];
      const char* old = data();
      for (std::size_t i = 0; i < len; ++i) fresh[i] = old[i];
      if (!is_short()) delete[] heap;
      heap = fresh;
      cap = next;
    }
    buf()[len++] = c;
    buf()[len] = '\0';
  }
  Str& operator+=(const Str& o) {
    for (char c : o) push(c);
    return *this;
  }
  const char* data() const { return is_short() ? local : heap; }
  std::size_t size() const { return len; }
  const char* begin() const { return data(); }
  const char* end() const { return data() + len; }
  bool operator==(const Str& o) const {
    if (len != o.len) return false;
    for (std::size_t i = 0; i < len; ++i)
      if (data()[i] != o.data()[i]) return false;
    return true;
  }
};

Str operator""_s(const char* s, std::size_t) { return Str(s); }
Str operator+(Str a, const Str& b) { return a += b; }

// Words of a text, as a range with its own iterator and an end marker.
struct EndOfWords {};
class WordIter {
  const char* p;
  const char* stop;
  void skip() { while (p != stop && *p == ' ') ++p; }
public:
  WordIter(const char* b, const char* e) : p(b), stop(e) { skip(); }
  Str operator*() const {
    Str w;
    for (const char* q = p; q != stop && *q != ' '; ++q) w.push(*q);
    return w;
  }
  WordIter& operator++() {
    while (p != stop && *p != ' ') ++p;
    skip();
    return *this;
  }
  bool operator==(EndOfWords) const { return p == stop; }
};
struct Words {
  const Str& text;
  WordIter begin() const { return {text.begin(), text.end()}; }
  EndOfWords end() const { return {}; }
};

template <class A, class B> struct Pair {
  A first;
  B second;
  template <std::size_t I> auto& get() {
    if constexpr (I == 0) return first;
    else return second;
  }
};
template <class A, class B> Pair(A, B) -> Pair<A, B>;
template <class A, class B> struct std::tuple_size<Pair<A, B>> {
  static constexpr std::size_t value = 2;
};
template <class A, class B> struct std::tuple_element<0, Pair<A, B>> { using type = A; };
template <class A, class B> struct std::tuple_element<1, Pair<A, B>> { using type = B; };

Pair<Str, int> longest(const Str& text) {
  Pair<Str, int> best{Str(""), 0};
  int index = 0;
  for (Str w : Words{text}) {
    if (w.size() > best.first.size()) best = {w, index};
    ++index;
  }
  return best;
}

int main() {
  int bad = 0;
  Str a = "short";
  Str b = "a string that is longer than fifteen";
  Str c = a;
  c += " and"_s + " more"_s;
  if (!(c == "short and more")) ++bad;
  if (!(b == Str(b)) || b.size() != 36) ++bad;

  Str moved = static_cast<Str&&>(b);
  if (b.size() != 0 || moved.size() != 36) ++bad;
  b = moved;
  if (!(b == moved)) ++bad;

  int words = 0;
  for (Str w : Words{moved}) {
    if (w.size() == 0) ++bad;
    ++words;
  }
  if (words != 7) ++bad;

  auto [word, at] = longest("the quick brown jumper hops"_s);
  if (!(word == "jumper") || at != 3) ++bad;

  Pair p{1, 2.5};
  auto& [x, y] = p;
  x = 4;
  if (p.first != 4 || y != 2.5) ++bad;
  return bad;
}
