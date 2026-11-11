// tidal-rules: DED-20 DED-2
// tidal-expect: accept
// tidal-note: I5-7, I5 programs p06 and p13. The explicit arguments give the pack two elements, which take the first two arguments. U is deduced from the third argument.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class... Ts, class U> Tag<void(Ts..., U)> lead(Ts..., U);
static_assert(same<decltype(lead<char, short>('a', 1, 2.0)), Tag<void(char, short, double)>>);
static_assert(same<decltype(lead(2.0)), Tag<void(double)>>);
