// tidal-rules: CTAD-27
// tidal-expect: accept
// tidal-note: The first base fixes the pack as X, Y, Z. One element remains for three bases, so Y and Z have no element and get empty lists.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
struct X {};
struct Y {};
struct Z {};
template <class... T> struct Bases : List<T...>, T... {};
Bases b{List<X, Y, Z>{}, X{}};
static_assert(same<decltype(b), Bases<X, Y, Z>>);
