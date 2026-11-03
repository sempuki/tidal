// W10: error codes beside exceptions.
// Go habit: func Parse(s string) (int, error).
// Here the low layer returns a value and a code and never throws.
// A middle layer turns a code into an exception. A boundary layer
// turns any exception back into a code.

namespace std {
class exception {               // LSUP-8
public:
  exception() noexcept {}
  virtual ~exception();
};
}

enum class Errc { ok, empty, bad_digit, overflow, unknown };

struct [[nodiscard]] Parsed {   // ATTR-5: a dropped result should warn
  int value;
  Errc err;
};

constexpr Parsed parse_int(const char* s) noexcept {
  if (s == nullptr || *s == '\0') return {0, Errc::empty};
  int v = 0;
  for (; *s != '\0'; ++s) {
    if (*s < '0' || *s > '9') return {0, Errc::bad_digit};
    int d = *s - '0';
    if (v > (2147483647 - d) / 10) return {0, Errc::overflow};
    v = v * 10 + d;
  }
  return {v, Errc::ok};
}

struct ParseError : std::exception {
  Errc err;
  explicit ParseError(Errc e) noexcept : err(e) {}
};

// Code to exception. The throw is on an untaken path in constant
// evaluation (CE-8, THR-10).
constexpr int parse_or_throw(const char* s) {
  auto [v, e] = parse_int(s);
  if (e != Errc::ok) throw ParseError(e);
  return v;
}

// Exception to code. The function is noexcept and catches everything
// inside itself (UNW-6). A try block is valid in constant evaluation
// while nothing throws (CE-1).
template <class F>
constexpr Parsed capture(F f) noexcept {
  try {
    return {f(), Errc::ok};
  } catch (const ParseError& e) {
    return {0, e.err};
  } catch (...) {
    return {0, Errc::unknown};
  }
}

static_assert(parse_int("123").value == 123);
static_assert(parse_int("").err == Errc::empty);
static_assert(parse_int("12x").err == Errc::bad_digit);
static_assert(parse_int("2147483647").value == 2147483647);
static_assert(parse_int("2147483648").err == Errc::overflow);
static_assert(parse_or_throw("42") == 42);
static_assert(capture([] { return parse_or_throw("7"); }).value == 7);

static_assert(noexcept(parse_int("1")));
static_assert(!noexcept(parse_or_throw("1")));
static_assert(noexcept(capture([] { return 1; })));

int sum_all(const char* const* items, int n) {
  int total = 0;
  for (int i = 0; i < n; ++i) total += parse_or_throw(items[i]);
  return total;
}

int main() {
  // Code path: check like `if err != nil`.
  if (Parsed p = parse_int("99"); p.err != Errc::ok || p.value != 99) return 1;
  auto [v, e] = parse_int("9a");
  if (e != Errc::bad_digit || v != 0) return 2;

  // Exception path: one handler covers a whole loop of calls.
  const char* good[] = {"1", "20", "300"};
  const char* bad[] = {"1", "", "3"};
  if (sum_all(good, 3) != 321) return 3;
  try {
    sum_all(bad, 3);
    return 4;
  } catch (const ParseError& pe) {
    if (pe.err != Errc::empty) return 5;
  }

  // Boundary: exceptions become codes again.
  Parsed c = capture([&] { return sum_all(bad, 3); });
  if (c.err != Errc::empty) return 6;
  Parsed u = capture([]() -> int { throw 5; });
  if (u.err != Errc::unknown) return 7;
  Parsed ok = capture([&] { return sum_all(good, 3); });
  if (ok.err != Errc::ok || ok.value != 321) return 8;
  return 0;
}
