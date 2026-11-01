// tidal-rules: DED-11
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assert
// tidal-note: A constant parameter declared with auto and deduced from an array bound gets the size type. GCC gives it int (Q-0209).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <auto N> Tag<decltype(N)> bound_type(int (&)[N]);
int arr[3];
static_assert(same<decltype(bound_type(arr)), Tag<decltype(sizeof 0)>>);
