// tidal-rules: DED-11
// tidal-expect: accept
// tidal-note: Constant parameters come from an array bound and from a class template argument. T comes from the type of the value.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T, int N> constexpr int count(T (&)[N]) { return N; }
template <int N> struct Fixed {};
template <int N> constexpr int fixed(Fixed<N>) { return N; }
template <auto V> struct Value {};
template <class T, T V> Tag<T> value_type(Value<V>);
int arr[3];
int grid[2][5];
static_assert(count(arr) == 3);
static_assert(count(grid) == 2);
static_assert(fixed(Fixed<4>{}) == 4);
static_assert(same<decltype(value_type(Value<'c'>{})), Tag<char>>);
