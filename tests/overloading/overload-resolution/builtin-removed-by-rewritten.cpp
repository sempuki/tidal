// tidal-rules: OVL-10
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assertion failed
// tidal-note: Q-0147. The rewritten candidate (red <=> blue) < 0 has the parameter list (Color, Color) of the built-in <. The built-in candidate is removed, so the rewritten one gives One. If the built-in stayed, it would win by OVL-32. GCC keeps it.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
enum Color { red, blue };
struct Order { friend One operator<(Order, int); };
Order operator<=>(Color, Color);
static_assert(same<decltype(red < blue), One>);
