// tidal-rules: RET-2 LAM-1 LAM-7 LAM-15 AUTO-4 DED-8 PACK-24 AUTO-10 DTY-2 MPTR-3
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: argument of type .auto &&. is incompatible
// tidal-note: Write suite W12, program compose.cpp (reports/clean-room/W12/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// tidal-note: Q-0354. EDG 7.0 checks the call g(static_cast<decltype(x)&&>(x)) before the generic lambda is instantiated.
// W12: a function that composes two functions, as Haskell's (.).

template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

// compose(f, g)(x) is f(g(x)). The closure holds copies of f and g
// (LAM-7). The parameter is a forwarding reference (AUTO-4), and the
// cast through decltype(x) passes the argument on with its value
// category (PACK-P3, PACK-24). decltype(auto) keeps a reference result
// (RET-2, AUTO-10).
template <class F, class G>
constexpr auto compose(F f, G g) {
  return [f, g](auto&& x) -> decltype(auto) {
    return f(g(static_cast<decltype(x)&&>(x)));
  };
}

// Many functions: compose_all(f, g, h)(x) is f(g(h(x))).
template <class F>
constexpr auto compose_all(F f) { return f; }

template <class F, class G, class... Rest>
constexpr auto compose_all(F f, G g, Rest... rest) {
  return compose(f, compose_all(g, rest...));
}

// A pointer to data member as a field accessor (MPTR-3).
template <class C, class T>
constexpr auto field(T C::* m) {
  return [m](const C& o) -> const T& { return o.*m; };
}

constexpr int twice(int x) { return 2 * x; }
constexpr auto inc = [](int x) { return x + 1; };
// id returns a copy. A forwarding id with decltype(auto) would return an
// rvalue reference to the temporary that g(x) makes inside compose, and
// that temporary ends with the return statement (OBJ-9, OBJ-2).
constexpr auto id = [](auto x) { return x; };

struct Point { int x; int y; };
struct Pair { int a; int b; };

constexpr bool laws() {
  auto f = compose(twice, inc);             // twice . inc
  auto g = compose(inc, twice);             // inc . twice
  bool order = f(3) == 8 && g(3) == 7;
  bool left_id = compose(id, inc)(4) == inc(4);
  bool right_id = compose(inc, id)(4) == inc(4);
  // associativity on one input
  auto h = [](int x) { return x * x; };
  bool assoc = compose(compose(inc, twice), h)(3) ==
               compose(inc, compose(twice, h))(3);
  bool many = compose_all(inc, twice, h, inc)(2) == 19;  // inc(2*(3*3))
  return order && left_id && right_id && assoc && many;
}
static_assert(laws());

constexpr bool fields() {
  Point p{3, 4};
  auto norm1 = compose([](int v) { return v < 0 ? -v : v; },
                       field(&Point::y));
  return norm1(p) == 4;
}
static_assert(fields());

// decltype(auto) passes a reference through the composition.
constexpr bool reference_result() {
  Pair pr{1, 2};
  auto first = [](Pair& p) -> int& { return p.a; };
  auto self = [](Pair& p) -> Pair& { return p; };
  auto get = compose(first, self);
  static_assert(same<decltype(get(pr)), int&>);
  get(pr) = 9;
  return pr.a == 9;
}
static_assert(reference_result());

// Composition of function pointers gives a closure with no captures of
// state except the two pointers.
constexpr bool pointers() {
  int (*p)(int) = twice;
  int (*q)(int) = inc;          // LAM-10
  auto r = compose(p, q);
  return r(10) == 22;
}
static_assert(pointers());

int main() {
  int failures = 0;
  if (!laws()) ++failures;
  if (!fields()) ++failures;
  if (!reference_result()) ++failures;
  if (!pointers()) ++failures;
  int n = 5;
  auto add_n = [&n](int x) { return x + n; };
  auto k = compose(add_n, twice);
  n = 7;                         // seen through the reference capture
  if (k(1) != 9) ++failures;
  return failures;
}
