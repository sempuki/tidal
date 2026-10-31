// tidal-rules: ICS-20
// tidal-expect: accept
// tidal-note: The list initializes a temporary of the referenced type, so int to long is a conversion and int to int the identity.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(const long&);
Two take(...);
static_assert(same<decltype(take({1})), One>);
One put(const long&);
Two put(const int&);
static_assert(same<decltype(put({1})), Two>);
