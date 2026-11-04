// tidal-rules: ICS-10
// tidal-expect: accept
// tidal-note: The built-in int& = int would need a user-defined sequence for its left operand, so it is not a candidate and the class's operator is used.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Cell { operator int&(); Cell& operator=(long); };
Cell c;
static_assert(same<decltype(c = 5), Cell&>);
