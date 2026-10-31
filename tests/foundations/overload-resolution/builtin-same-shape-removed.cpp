// tidal-rules: OVL-10
// tidal-expect: accept
// tidal-note: The non-member has the parameter list of the built-in candidate for Color, so the built-in candidate is removed. Without the removal the two would tie.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum Color { red, blue };
One operator==(Color, Color);
static_assert(same<decltype(red == blue), One>);
