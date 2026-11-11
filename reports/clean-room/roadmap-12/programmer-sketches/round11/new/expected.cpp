// Expected<T, E>: a value or an error in a union, built with
// std::construct_at so that it works in constant evaluation, Unexpected<E>
// with class template argument deduction, value() and error() through an
// explicit object parameter for every value category, and_then,
// transform, and or_else, a partial specialization for void, and a
// parser built from these steps.
namespace std {
using size_t = decltype(sizeof 0);
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> struct remove_const { using type = T; };
template <class T> struct remove_const<const T> { using type = T; };
template <class T> using remove_cvref_t = typename remove_const<typename remove_ref<T>::type>::type;
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

// Second try: Unexpected<E>{...} inside the member templates, since the
// front end rejects Unexpected{...} there (TR-12).
template <class E> struct Unexpected {
  E error;
};

struct BadAccess {};

template <class T, class E> class Expected {
  bool has_;
  union {
    T val_;
    E err_;
  };
  template <class, class> friend class Expected;
public:
  using value_type = T;
  using error_type = E;
  constexpr Expected(const T& v) : has_(true), val_(v) {}
  constexpr Expected(T&& v) : has_(true), val_(move(v)) {}
  template <class G> constexpr Expected(Unexpected<G> u) : has_(false), err_(move(u.error)) {}
  constexpr Expected(const Expected& o) : has_(o.has_) {
    if (has_) std::construct_at(&val_, o.val_); else std::construct_at(&err_, o.err_);
  }
  constexpr Expected(Expected&& o) : has_(o.has_) {
    if (has_) std::construct_at(&val_, move(o.val_)); else std::construct_at(&err_, move(o.err_));
  }
  Expected& operator=(const Expected&) = delete;
  constexpr ~Expected() { if (has_) val_.~T(); else err_.~E(); }

  constexpr bool has_value() const { return has_; }
  constexpr explicit operator bool() const { return has_; }

  template <class Self> constexpr auto&& value(this Self&& self) {
    if (!self.has_) throw BadAccess{};
    return forward<Self>(self).val_;
  }
  template <class Self> constexpr auto&& error(this Self&& self) { return forward<Self>(self).err_; }
  template <class Self, class U> constexpr T value_or(this Self&& self, U&& alt) {
    return self.has_ ? forward<Self>(self).val_ : static_cast<T>(forward<U>(alt));
  }

  template <class Self, class F> constexpr auto and_then(this Self&& self, F&& f) {
    using R = remove_cvref_t<decltype(f(forward<Self>(self).val_))>;
    static_assert(same<typename R::error_type, E>);
    if (self.has_) return forward<F>(f)(forward<Self>(self).val_);
    return R(Unexpected<E>{forward<Self>(self).err_});
  }
  template <class Self, class F> constexpr auto transform(this Self&& self, F&& f) {
    using U = remove_cvref_t<decltype(f(forward<Self>(self).val_))>;
    if (self.has_) return Expected<U, E>(forward<F>(f)(forward<Self>(self).val_));
    return Expected<U, E>(Unexpected<E>{forward<Self>(self).err_});
  }
  template <class Self, class F> constexpr Expected or_else(this Self&& self, F&& f) {
    if (self.has_) return Expected(forward<Self>(self).val_);
    return forward<F>(f)(forward<Self>(self).err_);
  }

  friend constexpr bool operator==(const Expected& a, const T& v) { return a.has_ && a.val_ == v; }
};

template <class E> class Expected<void, E> {
  bool has_;
  E err_;
public:
  constexpr Expected() : has_(true), err_() {}
  template <class G> constexpr Expected(Unexpected<G> u) : has_(false), err_(move(u.error)) {}
  constexpr explicit operator bool() const { return has_; }
  constexpr const E& error() const { return err_; }
};

enum class Err { empty, not_digit, overflow };

constexpr Expected<int, Err> digit(char c) {
  if (c < '0' || c > '9') return Unexpected{Err::not_digit};
  return c - '0';
}
constexpr Expected<int, Err> parse(const char* s) {
  if (!*s) return Unexpected{Err::empty};
  int n = 0;
  for (; *s; ++s) {
    auto d = digit(*s);
    if (!d) return Unexpected{d.error()};
    if (n > 214748364) return Unexpected{Err::overflow};
    n = n * 10 + d.value();
  }
  return n;
}
constexpr Expected<void, Err> check_even(int n) {
  if (n % 2) return Unexpected{Err::not_digit};
  return {};
}
constexpr Expected<int, Err> half(int n) {
  if (n % 2) return Unexpected{Err::not_digit};
  return n / 2;
}

static_assert(parse("123") == 123);
static_assert(!parse("") && parse("").error() == Err::empty);
static_assert(parse("12x").error() == Err::not_digit);
static_assert(parse("48").and_then(half).and_then(half) == 12);
static_assert(parse("6").and_then(half).and_then(half).error() == Err::not_digit);
static_assert(parse("21").transform([](int n) { return n * 2.0; }).value() == 42.0);
static_assert(parse("q").or_else([](Err) { return Expected<int, Err>(0); }) == 0);
static_assert(parse("7").value_or(1) == 7 && parse("").value_or(1) == 1);
static_assert(check_even(4) && !check_even(3));

inline int live = 0;
struct Text {
  char c[8] = {};
  Text(const char* s) { for (int i = 0; i < 7 && s[i]; ++i) c[i] = s[i]; ++live; }
  Text(const Text& o) : Text(o.c) {}
  Text(Text&& o) : Text(o.c) { o.c[0] = 0; }
  ~Text() { --live; }
};

Expected<Text, int> lookup(int key) {
  if (key == 1) return Text("one");
  if (key == 2) return Text("two");
  return Unexpected{key};
}

int main() {
  int fails = 0;
  {
    auto a = lookup(1);
    Text& t = a.value();
    if (t.c[0] != 'o') ++fails;
    const auto& ca = a;
    const Text& ct = ca.value();
    if (&ct != &t) ++fails;
    Text moved = move(a).value();
    if (moved.c[1] != 'n' || t.c[0] != 0) ++fails;
    auto b = lookup(5);
    if (b || b.error() != 5) ++fails;
    try {
      (void)b.value();
      ++fails;
    } catch (BadAccess) {
    }
    auto c = lookup(2).transform([](const Text& x) { return x.c[2]; });
    if (!(c == 'o')) ++fails;
    auto d = lookup(9).or_else([](int k) { return lookup(k - 7); });
    if (!d || d.value().c[1] != 'w') ++fails;
    auto copy = d;
    if (copy.value().c[0] != 't') ++fails;
  }
  if (live != 0) ++fails;
  return fails;
}
