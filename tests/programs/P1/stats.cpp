// tidal-rules: CLS-13 CPT-17 CPT-33 CTAD-8 DCL-19 LAM-1 PRIM-9
// tidal-expect: run
// tidal-note: Port suite P1, Rust persona, program stats.cpp (reports/clean-room/P1/report.md). The rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Port of rust/stats.rs: a generic function with a trait bound,
// closures, and an Option result.
//   K: PartialOrd, F: Fn(&T) -> K
//                 -> a concept with a compound requirement whose type
//                    constraint checks the key type (CPT-33, CPT-17)
//   &[T]          -> Slice<T>, a pointer and a length; a slice of an array
//                    deduces T from a constructor (CTAD-8); an empty
//                    slice has no array, because DCL-19 rejects a bound of 0
//   Option<T>     -> a class template; map is a method generic over Self
//                    through an explicit object parameter (CLS-13)
//   closures      -> lambdas, generic where Rust infers (LAM-1, LAM-P1)
//   &'static str == -> Str, compared by content, because PRIM-9 leaves
//                    the address of a string literal unspecified

using size_t = decltype(sizeof 0);
using u32 = unsigned int;

struct Str {
  const char* p;
  friend constexpr bool operator==(Str a, Str b) {
    const char* x = a.p;
    const char* y = b.p;
    while (*x != '\0' && *x == *y) { ++x; ++y; }
    return *x == *y;
  }
};

template <class T> class Slice {
  const T* data_ = nullptr;
  size_t len_ = 0;
public:
  constexpr Slice() = default;
  template <size_t N> constexpr Slice(const T (&a)[N]) : data_(a), len_(N) {}
  constexpr const T* begin() const { return data_; }
  constexpr const T* end() const { return data_ + len_; }
};

template <class T> class Option {
  bool has_ = false;
  T val_{};
public:
  constexpr Option() = default;
  constexpr Option(T v) : has_(true), val_(v) {}
  constexpr bool is_some() const { return has_; }
  constexpr bool is_none() const { return !has_; }
  constexpr T unwrap() const { return val_; }
  template <class F>
  constexpr auto map(this const auto& self, F f) {
    using R = decltype(f(self.val_));
    if (self.has_) return Option<R>(f(self.val_));
    return Option<R>();
  }
  friend constexpr bool operator==(const Option& a, const Option& b) {
    if (a.has_ != b.has_) return false;
    return !a.has_ || a.val_ == b.val_;
  }
};

template <class K>
concept PartialOrd = requires(const K& a, const K& b) { bool(a >= b); };

template <class F, class T>
concept KeyFn = requires(const F& f, const T& t) { { f(t) } -> PartialOrd; };

template <class T, KeyFn<T> F>
constexpr Option<const T*> max_by_key(Slice<T> items, F key) {
  Option<const T*> best;
  for (const T& it : items) {
    if (best.is_some() && key(*best.unwrap()) >= key(it)) continue;
    best = Option<const T*>(&it);
  }
  return best;
}

template <class P, class F>
constexpr u32 sum_filter_map(u32 lo, u32 hi, P pred, F f) {
  u32 total = 0;
  for (u32 n = lo; n <= hi; ++n)
    if (pred(n)) total += f(n);
  return total;
}

struct Word { Str text; u32 count; };

constexpr bool run() {
  Word words[] = {{{"a"}, 3}, {{"b"}, 7}, {{"c"}, 5}};
  auto top = max_by_key(Slice(words), [](const auto& w) { return w.count; })
                 .map([](const Word* w) { return w->text; });
  bool none = max_by_key(Slice<Word>(), [](const auto& w) { return w.count; }).is_none();
  u32 squares = sum_filter_map(1, 4, [](auto n) { return n % 2 == 0; },
                               [](auto n) { return n * n; });
  return top == Option<Str>(Str{"b"}) && none && squares == 20;
}

static_assert(run());

int main() { return run() ? 0 : 1; }
