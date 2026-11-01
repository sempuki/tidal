// tidal-rules: DED-22
// tidal-expect: accept
// tidal-note: The bound would give T the size type, but the second argument gives short, and the bound yields.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T, T N> Tag<T> sized(int (&)[N], T);
int arr[3];
static_assert(same<decltype(sized(arr, short(1))), Tag<short>>);
