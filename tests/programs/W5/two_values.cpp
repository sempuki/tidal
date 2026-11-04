// tidal-rules: RET-7 SB-3 SB-4 SB-5 SB-17 LSUP-3 SB-15 RET-2 AUTO-10 DTY-2
// tidal-expect: run
// tidal-note: Write suite W5, program two_values.cpp (reports/clean-room/W5/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A function that returns two values.

using size_t = decltype(sizeof 0);

struct Division {
  int quot;
  int rem;
};

constexpr Division divide(int a, int b) { return {a / b, a % b}; }

// A class that uses the tuple protocol.
namespace std {
template <class T> struct tuple_size;
template <size_t I, class T> struct tuple_element;
}  // namespace std

class Range {
 public:
  constexpr Range(int lo, int hi) : lo_(lo), hi_(hi) {}
  template <size_t I> constexpr int get() const {
    if constexpr (I == 0) return lo_;
    else return hi_;
  }

 private:
  int lo_;
  int hi_;
};

template <> struct std::tuple_size<Range> {
  static constexpr size_t value = 2;
};
template <size_t I> struct std::tuple_element<I, Range> {
  using type = int;
};
// SB-5 asks for tuple_size of the hidden variable's own type, so a
// const hidden variable needs specializations for const Range too.
template <> struct std::tuple_size<const Range> {
  static constexpr size_t value = 2;
};
template <size_t I> struct std::tuple_element<I, const Range> {
  using type = const int;
};

constexpr Range bounds(int a, int b) {
  if (a < b) return Range(a, b);
  return Range(b, a);
}

// Out-parameters through references.
constexpr void split(int v, int& tens, int& ones) {
  tens = v / 10;
  ones = v % 10;
}

// decltype(auto) on a parenthesized member gives a reference.
constexpr decltype(auto) quot_of(Division& d) { return (d.quot); }

template <class A, class B> constexpr bool same_type = false;
template <class A> constexpr bool same_type<A, A> = true;

constexpr bool check() {
  auto [q, r] = divide(17, 5);
  if (q != 3 || r != 2) return false;
  static_assert(same_type<decltype(q), int>);

  auto [lo, hi] = bounds(9, 4);
  if (lo != 4 || hi != 9) return false;

  int t = 0, o = 0;
  split(47, t, o);
  if (t != 4 || o != 7) return false;

  Division d{1, 0};
  quot_of(d) = 8;
  static_assert(same_type<decltype(quot_of(d)), int&>);
  return d.quot == 8;
}
static_assert(check());

int main(int argc, char**) {
  auto [q, r] = divide(10 + argc, 4);   // 11 / 4 when run plainly
  if (q != 2 || r != 3) return 1;
  const auto& [lo, hi] = bounds(argc + 5, argc);
  if (lo != 1 || hi != 6) return 2;
  return check() ? 0 : 3;
}
