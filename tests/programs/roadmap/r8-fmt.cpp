// tidal-rules: CEV-1 CE-2 PACK-13 OPR-8
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer sketch fmt.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A checked format string: a consteval constructor counts the "{}"
// placeholders of a string literal against the argument pack, as a
// format library does. Arguments are written into a fixed buffer with
// 'if constexpr' on own traits, a fold over the comma operator, and a
// user-defined literal for the buffer size.
namespace std {
using size_t = decltype(sizeof 0);
}

template <class T> struct type_identity { using type = T; };
template <class T> using type_identity_t = typename type_identity<T>::type;

template <class T> struct remove_cvref { using type = T; };
template <class T> struct remove_cvref<T&> : remove_cvref<T> {};
template <class T> struct remove_cvref<T&&> : remove_cvref<T> {};
template <class T> struct remove_cvref<const T> : remove_cvref<T> {};
template <class T> using remove_cvref_t = typename remove_cvref<T>::type;

template <class T, class U> inline constexpr bool is_same_v = false;
template <class T> inline constexpr bool is_same_v<T, T> = true;

void format_error(const char*);  // not constexpr: a call stops translation

template <class... Args>
struct format_string {
  const char* text;
  std::size_t length;

  template <std::size_t N>
  consteval format_string(const char (&s)[N]) : text(s), length(N - 1) {
    std::size_t holes = 0;
    for (std::size_t i = 0; i + 1 < N; ++i) {
      if (s[i] == '{') {
        if (s[i + 1] != '}') format_error("expected '}'");
        ++holes;
        ++i;
      }
    }
    if (holes != sizeof...(Args)) format_error("wrong number of arguments");
  }
};

class Buffer {
  char data_[64]{};
  std::size_t used_ = 0;
public:
  constexpr void put(char c) { if (used_ < sizeof data_ - 1) data_[used_++] = c; }
  constexpr void put(const char* s) { while (*s) put(*s++); }
  constexpr void put(long v) {
    if (v < 0) { put('-'); v = -v; }
    char digits[20];
    int n = 0;
    do { digits[n++] = static_cast<char>('0' + v % 10); v /= 10; } while (v);
    while (n) put(digits[--n]);
  }
  constexpr const char* str() const { return data_; }
  constexpr std::size_t size() const { return used_; }
};

template <class T>
constexpr void write_one(Buffer& out, const T& value) {
  using U = remove_cvref_t<T>;
  if constexpr (is_same_v<U, bool>) out.put(value ? "true" : "false");
  else if constexpr (is_same_v<U, char>) out.put(value);
  else if constexpr (is_same_v<U, const char*> || is_same_v<U, char*>) out.put(static_cast<const char*>(value));
  else out.put(static_cast<long>(value));
}

template <class... Args>
constexpr Buffer format(format_string<type_identity_t<Args>...> fmt, const Args&... args) {
  Buffer out;
  const char* p = fmt.text;
  auto next_hole = [&] {
    while (*p && *p != '{') out.put(*p++);
    if (*p) p += 2;
  };
  ((next_hole(), write_one(out, args)), ...);
  while (*p) out.put(*p++);
  return out;
}

constexpr bool equal(const char* a, const char* b) {
  while (*a && *a == *b) { ++a; ++b; }
  return *a == *b;
}

consteval std::size_t operator""_cap(unsigned long long n) { return static_cast<std::size_t>(n); }

static_assert(equal(format("x={}, y={}", 3, -12).str(), "x=3, y=-12"));
static_assert(equal(format("{} and {}", true, 'c').str(), "true and c"));
static_assert(format("no holes").size() == 8);
static_assert(64_cap == 64);

int main() {
  const char* name = "tidal";
  int count = 42;
  Buffer b = format("{}: {} items", name, count);
  if (!equal(b.str(), "tidal: 42 items")) return 1;
  // format("{} {}", count);  // does not compile: wrong number of arguments
  unsigned short small = 7;
  return equal(format("[{}]", small).str(), "[7]") ? 0 : 2;
}
