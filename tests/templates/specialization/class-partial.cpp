// tidal-rules: TSP-9
// tidal-expect: accept
// tidal-note: The partial specialization has its own parameter and its own members.
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<T*> { static constexpr int value = 1; using pointee = T; };
static_assert(Kind<int>::value == 0);
static_assert(Kind<int*>::value == 1);
static_assert(sizeof(Kind<char*>::pointee) == 1);
