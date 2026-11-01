// tidal-rules: DED-17
// tidal-expect: accept
// tidal-note: The bound comes from the number of elements, and T from each element.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T, int N> constexpr int len(const T (&)[N]) { return N; }
template <class T, int N> Tag<T> elem(T (&&)[N]);
template <class T, int M, int N> constexpr int area(const T (&)[M][N]) { return M * 10 + N; }
static_assert(len({1, 2, 3}) == 3);
static_assert(same<decltype(elem({'a', 'b'})), Tag<char>>);
static_assert(area({{1, 2}, {3, 4}, {5, 6}}) == 32);
