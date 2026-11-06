// An Optional<T> in a union: an in-place constructor that expands a pack in
// a member initializer, emplace through a placement new, a converting
// constructor from Optional<U>, a constructor from a nullopt tag type,
// value_or and transform with a deduced object parameter, explicit operator
// bool, defaulted-out comparison written as a hidden friend, the conditional
// operator with class operands of different types, and the same checks in
// constant evaluation, with a self-declared std::construct_at (LSUP-10).
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> struct remove_cvref { using type = T; };
template <class T> struct remove_cvref<const T> { using type = T; };
template <class T> struct remove_cvref<T&> : remove_cvref<T> {};
template <class T> struct remove_cvref<T&&> : remove_cvref<T> {};
template <class T> using remove_cvref_t = typename remove_cvref<T>::type;
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept {
  return static_cast<remove_ref_t<T>&&>(t);
}
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept {
  return static_cast<T&&>(t);
}
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class From, class To>
concept convertible = requires(From (&f)()) { [](To) {}(f()); };

struct in_place_t { explicit in_place_t() = default; };
inline constexpr in_place_t in_place{};
struct nullopt_t { constexpr explicit nullopt_t(int) {} };
inline constexpr nullopt_t nullopt{0};

template <class T>
class Optional {
  union { T val_; };
  bool has_ = false;
public:
  constexpr Optional() noexcept {}
  constexpr Optional(nullopt_t) noexcept {}
  template <class... A>
  constexpr explicit Optional(in_place_t, A&&... a) : val_(forward<A>(a)...), has_(true) {}
  template <class U = T>
    requires(!same<remove_cvref_t<U>, Optional> && !same<remove_cvref_t<U>, in_place_t> &&
             !same<remove_cvref_t<U>, nullopt_t> && convertible<U, T>)
  constexpr Optional(U&& u) : val_(forward<U>(u)), has_(true) {}
  template <class U>
    requires(!same<U, T> && convertible<const U&, T>)
  constexpr Optional(const Optional<U>& o) {
    if (o) { std::construct_at(&val_, *o); has_ = true; }
  }
  constexpr Optional(const Optional& o) {
    if (o.has_) { std::construct_at(&val_, o.val_); has_ = true; }
  }
  constexpr Optional& operator=(const Optional& o) {
    if (this != &o) { reset(); if (o.has_) emplace(o.val_); }
    return *this;
  }
  constexpr ~Optional() { reset(); }

  constexpr void reset() {
    if (has_) { val_.~T(); has_ = false; }
  }
  template <class... A>
  constexpr T& emplace(A&&... a) {
    reset();
    T* p = std::construct_at(&val_, forward<A>(a)...);
    has_ = true;
    return *p;
  }
  constexpr explicit operator bool() const noexcept { return has_; }
  constexpr const T& operator*() const& { return val_; }
  constexpr T& operator*() & { return val_; }

  template <class Self, class U>
  constexpr T value_or(this Self&& self, U&& fallback) {
    return self.has_ ? T(forward<Self>(self).val_) : T(forward<U>(fallback));
  }
  template <class Self, class F>
  constexpr auto transform(this Self&& self, F&& f) -> Optional<remove_cvref_t<decltype(f(forward<Self>(self).val_))>> {
    if (!self.has_) return nullopt;
    return Optional<remove_cvref_t<decltype(f(forward<Self>(self).val_))>>(in_place, f(forward<Self>(self).val_));
  }
  friend constexpr bool operator==(const Optional& a, const Optional& b) {
    if (a.has_ != b.has_) return false;
    return !a.has_ || a.val_ == b.val_;
  }
};

struct Point {
  int x, y;
  constexpr Point(int a, int b) : x(a), y(b) {}
  constexpr bool operator==(const Point&) const = default;
};

constexpr int run(bool flag) {
  Optional<Point> p(in_place, 3, 4);           // pack in the member initializer
  if (!p || (*p).x != 3) return 1;
  Optional<Point> q;
  q.emplace(5, 6);                             // pack in a placement new
  if (!(q == Optional<Point>(in_place, 5, 6))) return 2;
  Optional<int> none = nullopt;
  if (none.value_or(7) != 7) return 3;
  Optional<long> widened = Optional<int>(41);  // converting constructor
  if (*widened != 41) return 4;
  auto len = p.transform([](const Point& pt) { return pt.x * pt.x + pt.y * pt.y; });
  if (*len != 25) return 5;
  // conditional operator: an Optional<int> lvalue against a nullopt_t lvalue,
  // and against an int prvalue; each converts to Optional<int>
  Optional<int> seven = 7;
  Optional<int> c1 = flag ? seven : nullopt;
  Optional<int> c2 = flag ? nullopt : seven;
  Optional<int> c3 = flag ? seven : 8;
  if (bool(c1) != flag || bool(c2) == flag || *c3 != (flag ? 7 : 8)) return 6;
  return 0;
}

static_assert(run(true) == 0);
static_assert(run(false) == 0);

int main() { return run(true) + run(false); }
