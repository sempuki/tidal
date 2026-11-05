// tidal-rules: ORD-13 ORD-14
// tidal-expect: accept
// tidal-note: The two partial specializations have the same pattern, so ordering ties, and the constraint decides for int*.
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<T*> { static constexpr int value = 1; };
template <class T> requires (sizeof(T) > 1) struct Kind<T*> { static constexpr int value = 2; };
static_assert(Kind<char*>::value == 1);
static_assert(Kind<int*>::value == 2);
