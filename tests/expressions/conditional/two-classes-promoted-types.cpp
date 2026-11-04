// tidal-rules: COND-11
// tidal-expect: accept
// tidal-note: The built-in candidates take promoted arithmetic types, so char, bool, and an unscoped enumeration give int. A pair of different types gives the type of the usual arithmetic conversions. A pointer pair and a member pointer pair use the candidate for one such type.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
enum Color { red };
struct B { int m; };
template <class T, int N> struct To { operator T(); };
extern bool c;
To<char, 1> ch1; To<char, 2> ch2;
To<bool, 1> b1; To<bool, 2> b2;
To<Color, 1> e1; To<Color, 2> e2;
To<int, 1> i1; To<double, 2> d1; To<long, 3> l1;
To<int*, 1> p1; To<const int*, 2> p2;
To<int B::*, 1> m1; To<int B::*, 2> m2;
static_assert(same<decltype(c ? ch1 : ch2), int>);
static_assert(same<decltype(c ? b1 : b2), int>);
static_assert(same<decltype(c ? e1 : e2), int>);
static_assert(same<decltype(c ? i1 : d1), double>);
static_assert(same<decltype(c ? i1 : l1), long>);
static_assert(same<decltype(c ? p1 : p2), const int*>);
static_assert(same<decltype(c ? m1 : m2), int B::*>);
