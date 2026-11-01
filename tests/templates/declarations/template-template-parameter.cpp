// tidal-rules: TPL-12
// tidal-expect: accept
// tidal-note: A template template parameter is used with arguments inside the template. class and typename both introduce it.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct List { using item = T; };
template <template <class> class Container> struct WithInt { using type = Container<int>; };
template <template <class> typename Container> struct WithChar { using type = Container<char>; };
static_assert(same<WithInt<List>::type, List<int>>);
static_assert(same<WithChar<List>::type::item, char>);
