// tidal-rules: TSP-10
// tidal-expect: accept
template <class T> constexpr bool is_pointer = false;
template <class T> constexpr bool is_pointer<T*> = true;
static_assert(!is_pointer<int>);
static_assert(is_pointer<int*>);
static_assert(is_pointer<const char**>);
