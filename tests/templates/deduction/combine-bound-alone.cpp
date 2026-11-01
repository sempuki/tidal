// tidal-rules: DED-22
// tidal-expect: accept
// tidal-note: No other pair gives T a value, so the bound gives T the type of sizeof, and N is 3.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T, T N> Tag<T> sized(int (&)[N]);
template <class T, T N> constexpr T bound(int (&)[N]) { return N; }
int arr[3];
static_assert(same<decltype(sized(arr)), Tag<decltype(sizeof 0)>>);
static_assert(bound(arr) == 3);
