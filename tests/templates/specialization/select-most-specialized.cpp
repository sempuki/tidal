// tidal-rules: TSP-15
// tidal-expect: accept
// tidal-note: const int* matches T* and const T*. The second partial specialization is more specialized.
template <class T> struct Kind { static constexpr int value = 0; };
template <class T> struct Kind<T*> { static constexpr int value = 1; };
template <class T> struct Kind<const T*> { static constexpr int value = 2; };
static_assert(Kind<int*>::value == 1);
static_assert(Kind<const int*>::value == 2);
