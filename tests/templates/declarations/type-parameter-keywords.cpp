// tidal-rules: TPL-5
// tidal-expect: accept
// tidal-note: class and typename declare the same kind of parameter, and the name is optional.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct ByClass { using type = T; };
template <typename T> struct ByTypename { using type = T; };
template <class> struct Unnamed { static constexpr int size = 1; };
template <class T> struct ByClass;
template <typename T> struct ByClass;
static_assert(same<ByClass<int>::type, ByTypename<int>::type>);
static_assert(Unnamed<double>::size == 1);
