// tidal-rules: TSP-9 TPL-30
// tidal-expect: accept
// tidal-note: The heads of the partial specializations declare a pack before a type parameter.
template <class... T> struct List { static constexpr int count = 0; };
template <class... Rest, class First> struct List<First, Rest...> { static constexpr int count = 1 + sizeof...(Rest); };
template <class T, class U> struct Pair { static constexpr int count = 0; };
template <class... A, class B> struct Pair<B, List<A...>> { static constexpr int count = sizeof...(A); };
static_assert(List<int, char, long>::count == 3);
static_assert(Pair<int, List<char, long>>::count == 2);
