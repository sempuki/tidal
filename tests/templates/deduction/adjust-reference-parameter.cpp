// tidal-rules: DED-7
// tidal-expect: accept
// tidal-note: A reference parameter matches its referred type. The argument keeps its const and does not decay. Top-level const on the parameter takes no part, and neither does the value category.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> ref(T&);
template <class T> Tag<T> cref(const T&);
template <class T> Tag<T> cval(const T);
template <class T> Tag<T> val(T);
int n = 0;
const int c = 1;
int arr[3];
static_assert(same<decltype(ref(c)), Tag<const int>>);
static_assert(same<decltype(ref(arr)), Tag<int[3]>>);
static_assert(same<decltype(cref(c)), Tag<int>>);
static_assert(same<decltype(cref(1)), Tag<int>>);
static_assert(same<decltype(cref(arr)), Tag<int[3]>>);
static_assert(same<decltype(cval(c)), Tag<int>>);
static_assert(same<decltype(val(static_cast<int&&>(n))), Tag<int>>);
