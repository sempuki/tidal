// A small range pipeline: a fixed vector filled from a braced list,
// filter and transform views with sentinels, adaptors joined by '|',
// own concepts with requires-expressions, constrained auto parameters,
// and class template argument deduction.
namespace std {
using size_t = decltype(sizeof 0);
template <class E> class initializer_list {
  const E* first_;
  size_t size_;
  constexpr initializer_list(const E* f, size_t n) : first_(f), size_(n) {}
public:
  constexpr initializer_list() : first_(nullptr), size_(0) {}
  constexpr const E* begin() const { return first_; }
  constexpr const E* end() const { return first_ + size_; }
  constexpr size_t size() const { return size_; }
};
}

template <class T, class U> concept same_as = __is_same(T, U) && __is_same(U, T);

template <class R>
concept range = requires(R& r) {
  r.begin();
  r.end();
  { r.begin() != r.end() } -> same_as<bool>;
};

template <class T>
concept integral = __is_integral(T);

template <class T, std::size_t Cap>
class FixedVec {
  T data_[Cap]{};
  std::size_t size_ = 0;
public:
  constexpr FixedVec(std::initializer_list<T> init) {
    for (const T& x : init) push(x);
  }
  constexpr void push(const T& x) { data_[size_++] = x; }
  constexpr const T* begin() const { return data_; }
  constexpr const T* end() const { return data_ + size_; }
  constexpr std::size_t size() const { return size_; }
};

struct end_mark {};

template <class R, class Pred>
class filter_view {
  const R* base_;
  Pred pred_;
public:
  constexpr filter_view(const R& r, Pred p) : base_(&r), pred_(p) {}
  struct iterator {
    const filter_view* v;
    decltype(base_->begin()) cur;
    constexpr void skip() {
      while (cur != v->base_->end() && !v->pred_(*cur)) ++cur;
    }
    constexpr decltype(auto) operator*() const { return *cur; }
    constexpr iterator& operator++() { ++cur; skip(); return *this; }
    friend constexpr bool operator==(const iterator& i, end_mark) {
      return i.cur == i.v->base_->end();
    }
  };
  constexpr iterator begin() const {
    iterator it{this, base_->begin()};
    it.skip();
    return it;
  }
  constexpr end_mark end() const { return {}; }
};

template <class R, class F>
class transform_view {
  R base_;
  F f_;
public:
  constexpr transform_view(R r, F f) : base_(r), f_(f) {}
  struct iterator {
    const transform_view* v;
    decltype(base_.begin()) cur;
    constexpr auto operator*() const { return v->f_(*cur); }
    constexpr iterator& operator++() { ++cur; return *this; }
    friend constexpr bool operator==(const iterator& i, end_mark e) {
      return i.cur == e;
    }
  };
  constexpr iterator begin() const { return {this, base_.begin()}; }
  constexpr end_mark end() const { return {}; }
};

template <class F> struct filter_fn { F f; };
template <class F> struct transform_fn { F f; };

template <class F> constexpr filter_fn<F> filter(F f) { return {f}; }
template <class F> constexpr transform_fn<F> transform(F f) { return {f}; }

template <range R, class F>
constexpr auto operator|(const R& r, filter_fn<F> a) { return filter_view(r, a.f); }

template <class R, class F>
constexpr auto operator|(R r, transform_fn<F> a) { return transform_view(r, a.f); }

constexpr auto sum(range auto const& r) {
  long total = 0;
  for (auto x : r) total += x;
  return total;
}

constexpr bool is_even(integral auto x) { return x % 2 == 0; }

constexpr long run() {
  FixedVec<int, 8> v{1, 2, 3, 4, 5, 6};
  auto squares = v | filter([](int x) { return is_even(x); })
                   | transform([](int x) { return long{x} * x; });
  return sum(squares);  // 4 + 16 + 36
}

static_assert(run() == 56);
static_assert(range<FixedVec<int, 2>>);
static_assert(!range<int>);

int main() {
  FixedVec<int, 4> w{7, 8, 9};
  int scale = 3;
  auto tripled = w | filter([](int x) { return x != 8; })
                   | transform([scale](int x) { return x * scale; });
  int n = 0;
  long last = 0;
  for (auto x : tripled) { ++n; last = x; }
  if (n != 2 || last != 27) return 1;
  if (sum(tripled) != 48) return 2;
  return run() == 56 ? 0 : 3;
}
