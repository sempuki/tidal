// tidal-rules: ORD-14
// tidal-expect: accept
// tidal-note: Each partial specialization orders as a function template with one parameter of its pattern. A repeated parameter is more specialized than two parameters.
template <class T, class U> struct Pair { static constexpr int value = 0; };
template <class T> struct Pair<T, T> { static constexpr int value = 1; };
template <class T, class U> struct Pair<T*, U> { static constexpr int value = 2; };
template <class T> struct Pair<T*, T*> { static constexpr int value = 3; };
static_assert(Pair<int, char>::value == 0);
static_assert(Pair<int, int>::value == 1);
static_assert(Pair<int*, char>::value == 2);
static_assert(Pair<int*, int*>::value == 3);
template <class T> constexpr int kind = 0;
template <class T> constexpr int kind<T*> = 1;
template <class T> constexpr int kind<T* const*> = 2;
static_assert(kind<int**> == 1);
static_assert(kind<int* const*> == 2);
