// tidal-rules: CTAD-27 CTAD-9
// tidal-expect: accept
// tidal-note: The first base List<T...> fixes T as X, Y, Z. The trailing pack of bases then has three elements, although the list gives only two more.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class... T> struct List {};
struct X {};
struct Y {};
struct Z {};
template <class... T> struct Bases : List<T...>, T... {};
Bases b{List<X, Y, Z>{}, {}, {}};
static_assert(same<decltype(b), Bases<X, Y, Z>>);
