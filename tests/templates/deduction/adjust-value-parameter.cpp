// tidal-rules: DED-6
// tidal-expect: accept
// tidal-note: For a parameter that is not a reference, arrays and functions decay, and top-level const goes.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> take(T);
int arr[3];
const int c = 1;
int fn(char);
const int* const cp = nullptr;
static_assert(same<decltype(take(arr)), Tag<int*>>);
static_assert(same<decltype(take(fn)), Tag<int (*)(char)>>);
static_assert(same<decltype(take(c)), Tag<int>>);
static_assert(same<decltype(take("ab")), Tag<const char*>>);
static_assert(same<decltype(take(cp)), Tag<const int*>>);
