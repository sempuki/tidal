// tidal-rules: CTAD-10 CTAD-9
// tidal-expect: accept
// tidal-note: The member int v[2] does not depend on T, so its braces are elided as in any aggregate initialization (LIST-4). 1 and 2 fill v, and 3L gives T = long.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Counts { int v[2]; T total; };
Counts c{1, 2, 3L};
static_assert(same<decltype(c), Counts<long>>);
