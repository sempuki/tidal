// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: const T matches a type with a top-level const. T* matches a pointer to a const type, with T = const int, and does not match a const pointer.
template <class T> struct IsConst { static constexpr bool value = false; };
template <class T> struct IsConst<const T> { static constexpr bool value = true; };
template <class T> struct IsPtr { static constexpr bool value = false; };
template <class T> struct IsPtr<T*> { static constexpr bool value = true; };
static_assert(IsConst<const int>::value);
static_assert(!IsConst<int>::value);
static_assert(!IsConst<const int*>::value);
static_assert(IsConst<int* const>::value);
static_assert(IsPtr<const int*>::value);
static_assert(!IsPtr<int* const>::value);
