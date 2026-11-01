// tidal-rules: CNS-17
// tidal-expect: accept
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> requires (sizeof(T) > 1) struct Kind<T> { static constexpr int value = 1; };
static_assert(Kind<char>::value == 0 && Kind<int>::value == 1);
