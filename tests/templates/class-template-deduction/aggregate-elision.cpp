// tidal-rules: CTAD-10
// tidal-expect: accept
// tidal-note: The braces of a member whose type is not dependent are elided, so 1 and 2 fill Plain and 3 gives T.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Plain { int x; int y; };
template <class T> struct In { T x; T y; };
template <class T> struct Outer { In<T> in; };
Outer a{In<int>{1, 2}};
template <class T> struct Mixed { Plain p; T z; };
Mixed b{1, 2, 3L};
static_assert(same<decltype(a), Outer<int>> && same<decltype(b), Mixed<long>>);
