// tidal-rules: CTAD-27 CTAD-9
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: class template argument deduction failed
// tidal-note: The first base fixes T as X, Y, Z, and the list in parentheses gives one more element. The bases Y and Z have no element and take the default argument. GCC forms no aggregate candidate from parentheses for a class with a pack of bases (Q-0352).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
struct X {};
struct Y {};
struct Z {};
template <class... T> struct Bases : List<T...>, T... {};
Bases b(List<X, Y, Z>{}, X{});
static_assert(same<decltype(b), Bases<X, Y, Z>>);
