// tidal-rules: TPL-28
// tidal-expect: accept
// tidal-note: First<int, char> is int, with the second argument dropped. A parameter Pointer<T> is T*, so a call with an int* deduces T.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, class U> using First = T;
template <class T> using Pointer = T*;
template <class T> constexpr T* take(Pointer<T> p) { return p; }
int n = 0;
static_assert(same<First<int, char>, int>);
static_assert(same<decltype(take(&n)), int*>);
