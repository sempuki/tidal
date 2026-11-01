// tidal-rules: TPL-11
// tidal-expect: accept
// tidal-note: The type of an auto parameter is the type of its argument.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <auto V> struct Value { using type = decltype(V); };
template <decltype(auto) V> struct Exact { using type = decltype(V); };
int object = 0;
static_assert(same<Value<1>::type, int>);
static_assert(same<Value<'c'>::type, char>);
static_assert(same<Value<2L>::type, long>);
static_assert(same<Value<&object>::type, int*>);
static_assert(same<Exact<(object)>::type, int&>);
