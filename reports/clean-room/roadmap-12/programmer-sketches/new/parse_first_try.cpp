// Parser combinators: a parser is any callable that takes a position in a
// string and returns a Result<T>. A concept Parser, the combinators map,
// alt (operator|), seq (operator>>), many with a fold, and chain_left for
// left-associative operators, then an arithmetic grammar with recursion
// through an ordinary function, checked in static_assert and at run time.
template <class T> T&& declval();
template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

template <class T>
struct Result {
  bool ok;
  T value;
  const char* rest;
  constexpr explicit operator bool() const { return ok; }
};
template <class T> constexpr Result<T> fail(const char* at) { return {false, T{}, at}; }

template <class P>
concept Parser = requires(P p, const char* s) {
  { p(s).ok } -> same_as_bool;
  p(s).value;
  p(s).rest;
};

template <class P> using value_t = decltype(declval<P&>()(nullptr).value);

template <class A, class B> struct Pair { A first; B second; };

constexpr auto ch(char c) {
  return [c](const char* s) -> Result<char> {
    if (*s == c) return {true, c, s + 1};
    return fail<char>(s);
  };
}

inline constexpr auto digit = [](const char* s) -> Result<int> {
  if (*s >= '0' && *s <= '9') return {true, *s - '0', s + 1};
  return fail<int>(s);
};

template <Parser P, class F>
constexpr auto map(P p, F f) {
  using U = decltype(f(declval<value_t<P>>()));
  return [=](const char* s) -> Result<U> {
    auto [ok, v, rest] = p(s);
    if (!ok) return fail<U>(s);
    return {true, f(v), rest};
  };
}

template <Parser P, Parser Q>
  requires same<value_t<P>, value_t<Q>>
constexpr auto operator|(P p, Q q) {
  return [=](const char* s) {
    auto r = p(s);
    return r ? r : q(s);
  };
}

template <Parser P, Parser Q>
constexpr auto operator>>(P p, Q q) {
  using R = Pair<value_t<P>, value_t<Q>>;
  return [=](const char* s) -> Result<R> {
    auto a = p(s);
    if (!a) return fail<R>(s);
    auto b = q(a.rest);
    if (!b) return fail<R>(s);
    return {true, R{a.value, b.value}, b.rest};
  };
}

// One or more of p, folded from the left with op, starting from init.
template <Parser P, class T, class Op>
constexpr auto many1(P p, T init, Op op) {
  return [=](const char* s) -> Result<T> {
    auto r = p(s);
    if (!r) return fail<T>(s);
    T acc = op(init, r.value);
    const char* at = r.rest;
    for (;;) {
      auto n = p(at);
      if (!n) break;
      acc = op(acc, n.value);
      at = n.rest;
    }
    return {true, acc, at};
  };
}

// operand (op operand)*, where op gives a function to apply.
template <Parser P, Parser O>
constexpr auto chain_left(P operand, O op) {
  using T = value_t<P>;
  return [=](const char* s) -> Result<T> {
    auto r = operand(s);
    if (!r) return r;
    T acc = r.value;
    const char* at = r.rest;
    for (;;) {
      auto o = op(at);
      if (!o) break;
      auto n = operand(o.rest);
      if (!n) break;
      acc = o.value(acc, n.value);
      at = n.rest;
    }
    return {true, acc, at};
  };
}

using BinOp = int (*)(int, int);
inline constexpr auto number = many1(digit, 0, [](int a, int d) { return a * 10 + d; });
inline constexpr auto add_op =
    map(ch('+'), [](char) -> BinOp { return [](int a, int b) { return a + b; }; }) |
    map(ch('-'), [](char) -> BinOp { return [](int a, int b) { return a - b; }; });
inline constexpr auto mul_op =
    map(ch('*'), [](char) -> BinOp { return [](int a, int b) { return a * b; }; }) |
    map(ch('/'), [](char) -> BinOp { return [](int a, int b) { return b ? a / b : 0; }; });

constexpr Result<int> expr(const char* s);

constexpr Result<int> parens(const char* s) {
  auto p = ch('(') >> expr >> ch(')');
  return map(p, [](auto v) { return v.first.second; })(s);
}
constexpr Result<int> factor(const char* s) { return (number | parens)(s); }
constexpr Result<int> term(const char* s) { return chain_left(factor, mul_op)(s); }
constexpr Result<int> expr(const char* s) { return chain_left(term, add_op)(s); }

constexpr int eval(const char* s) {
  auto r = expr(s);
  return r && *r.rest == 0 ? r.value : -1;
}

static_assert(Parser<decltype(digit)>);
static_assert(!Parser<int>);
static_assert(same<value_t<decltype(number)>, int>);
static_assert(eval("42") == 42);
static_assert(eval("2*(3+4)-5") == 9);
static_assert(eval("100/7/2") == 7);
static_assert(eval("1-2-3") == -4);
static_assert(eval("(1") == -1);
static_assert(eval("8*") == -1);

int main() {
  int fails = 0;
  const char* cases[] = {"1+2*3", "((7))", "9-3*2+1", "12/(1+1)/3", "x"};
  int expect[] = {7, 7, 4, 2, -1};
  for (int i = 0; i < 5; ++i)
    if (eval(cases[i]) != expect[i]) ++fails;
  auto pair = (digit >> ch(':') >> digit)("4:5");
  if (!pair || pair.value.first.first + pair.value.second != 9) ++fails;
  auto none = (ch('a') | ch('b'))("c");
  if (none || none.rest[0] != 'c') ++fails;
  return fails;
}
