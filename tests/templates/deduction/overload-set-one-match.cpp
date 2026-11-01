// tidal-rules: DED-19
// tidal-expect: accept
// tidal-note: Only twice(int) matches T (*)(int), so the set gives T = int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> apply(T (*)(int));
int twice(int);
long twice(long);
static_assert(same<decltype(apply(twice)), Tag<int>>);
