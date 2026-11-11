// tidal-rules: CTAD-1 CTAD-11 DED-34 LAM-15
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer sketch views.cpp, last version (reports/clean-room/roadmap-11/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Lazy range adaptors: an Iota range, a Span over an array with a
// deduction guide, Filter, Transform, and Take views over any range whose
// end can be a sentinel of another type, adaptor closures joined with a
// pipe operator, and algorithms over an iterator and a sentinel. The
// pipelines also run in constant evaluation.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> struct remove_const { using type = T; };
template <class T> struct remove_const<const T> { using type = T; };
template <class T> using remove_cvref_t = typename remove_const<remove_ref_t<T>>::type;
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }
template <class T> T&& declval() noexcept;

template <class R> concept Range = requires(const R& r) { r.begin(); r.end(); };
template <class R> using iterator_t = decltype(declval<const R&>().begin());
template <class R> using sentinel_t = decltype(declval<const R&>().end());

struct Iota {
  int lo, hi;
  struct Iter {
    int v;
    constexpr int operator*() const { return v; }
    constexpr Iter& operator++() { ++v; return *this; }
    constexpr bool operator==(const Iter&) const = default;
  };
  constexpr Iter begin() const { return {lo}; }
  constexpr Iter end() const { return {hi}; }
};

template <class T> struct Span {
  T* p;
  std::size_t n;
  // Second try: the first try had no constructor, so Span(data) was an
  // aggregate initialized in parentheses with n = 0 (my mistake).
  template <std::size_t N> constexpr Span(T (&a)[N]) : p(a), n(N) {}
  constexpr T* begin() const { return p; }
  constexpr T* end() const { return p + n; }
};
template <class T, std::size_t N> Span(T (&)[N]) -> Span<T>;

struct End {};

template <Range R, class P> class Filter {
  R base_;
  P pred_;
public:
  constexpr Filter(R r, P p) : base_(move(r)), pred_(move(p)) {}
  class Iter {
    iterator_t<R> cur_;
    sentinel_t<R> end_;
    const P* pred_;
    constexpr void skip() { while (cur_ != end_ && !(*pred_)(*cur_)) ++cur_; }
  public:
    constexpr Iter(iterator_t<R> c, sentinel_t<R> e, const P* p) : cur_(c), end_(e), pred_(p) { skip(); }
    constexpr decltype(auto) operator*() const { return *cur_; }
    constexpr Iter& operator++() { ++cur_; skip(); return *this; }
    constexpr bool operator==(End) const { return cur_ == end_; }
  };
  constexpr Iter begin() const { return Iter(base_.begin(), base_.end(), &pred_); }
  constexpr End end() const { return {}; }
};

template <Range R, class F> class Transform {
  R base_;
  F f_;
public:
  constexpr Transform(R r, F f) : base_(move(r)), f_(move(f)) {}
  class Iter {
    iterator_t<R> cur_;
    const F* f_;
  public:
    constexpr Iter(iterator_t<R> c, const F* f) : cur_(c), f_(f) {}
    constexpr auto operator*() const { return (*f_)(*cur_); }
    constexpr Iter& operator++() { ++cur_; return *this; }
    constexpr bool operator==(const sentinel_t<R>& s) const { return cur_ == s; }
  };
  constexpr Iter begin() const { return Iter(base_.begin(), &f_); }
  constexpr sentinel_t<R> end() const { return base_.end(); }
};

template <Range R> class Take {
  R base_;
  int count_;
public:
  constexpr Take(R r, int n) : base_(move(r)), count_(n) {}
  struct Sent { sentinel_t<R> end; };
  class Iter {
    iterator_t<R> cur_;
    int left_;
  public:
    constexpr Iter(iterator_t<R> c, int n) : cur_(c), left_(n) {}
    constexpr decltype(auto) operator*() const { return *cur_; }
    constexpr Iter& operator++() { ++cur_; --left_; return *this; }
    constexpr bool operator==(const Sent& s) const { return left_ == 0 || cur_ == s.end; }
  };
  constexpr Iter begin() const { return Iter(base_.begin(), count_); }
  constexpr Sent end() const { return Sent{base_.end()}; }
};

// Fifth try: filter and transform name their template parameter, since
// the front end stops on a generic lambda in an abbreviated template.
// Fourth try: Adaptor(...) in parentheses, since the front end rejects
// Adaptor{...} inside a function template (TR-12).
template <class F> struct Adaptor { F make; };
template <Range R, class F> constexpr auto operator|(R r, const Adaptor<F>& a) { return a.make(move(r)); }

template <class P> constexpr auto filter(P pred) {
  return Adaptor([pred](auto r) { return Filter(move(r), pred); });
}
template <class F> constexpr auto transform(F f) {
  return Adaptor([f](auto r) { return Transform(move(r), f); });
}
constexpr auto take(int n) {
  return Adaptor([n](auto r) { return Take(move(r), n); });
}

template <class I, class S, class T, class Op> constexpr T fold(I first, S last, T init, Op op) {
  for (; first != last; ++first) init = op(move(init), *first);
  return init;
}
template <Range R> constexpr int sum(const R& r) {
  return fold(r.begin(), r.end(), 0, [](int a, int b) { return a + b; });
}
template <Range R> constexpr int count(const R& r) {
  int n = 0;
  for (auto&& x : r) { (void)x; ++n; }
  return n;
}

constexpr int squares_of_evens(int n) {
  auto v = Iota{0, n} | filter([](int x) { return x % 2 == 0; })
                      | transform([](int x) { return x * x; })
                      | take(3);
  return sum(v);
}
static_assert(squares_of_evens(100) == 0 + 4 + 16);
static_assert(squares_of_evens(3) == 0 + 4);
static_assert(count(Iota{0, 10} | take(20)) == 10);

struct Point { int x, y; };

int main() {
  int fails = 0;
  int data[] = {5, 3, 8, 1, 9, 2};
  auto big = Span(data) | filter([](int x) { return x > 2; });
  int total = 0;
  for (int x : big) total += x;
  if (total != 5 + 3 + 8 + 9) ++fails;

  int limit = 4;
  auto doubled = Span(data) | transform([&limit](int x) { return x < limit ? 2 * x : x; });
  if (sum(doubled) != 5 + 6 + 8 + 2 + 9 + 4) ++fails;

  Point pts[] = {{1, 2}, {3, 4}, {5, 6}};
  auto xs = Span(pts) | transform([](const Point& p) { return p.x; }) | take(2);
  if (sum(xs) != 4 || count(xs) != 2) ++fails;

  for (int& x : Span(data) | take(2)) x = 0;
  if (data[0] != 0 || data[1] != 0 || data[2] != 8) ++fails;
  return fails;
}
