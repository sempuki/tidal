// tidal-rules: CLS-8
// tidal-expect: accept
// tidal-note: A nested class defined outside its class, a scoped enumeration, and a typedef.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Outer {
  struct Inner;
  enum class Color { red, green };
  typedef long Count;
};
struct Outer::Inner { Outer::Count n; };
constexpr Outer::Inner value{5};
static_assert(value.n == 5);
static_assert(same<Outer::Count, long>);
static_assert(static_cast<int>(Outer::Color::green) == 1);
