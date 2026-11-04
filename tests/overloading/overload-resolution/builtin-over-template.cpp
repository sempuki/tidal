// tidal-rules: OVL-27 OVL-9
// tidal-expect: accept
// tidal-note: The template gives operator==<Color>(Color, Color), with the same sequences as the built-in candidate. The built-in candidate counts as a plain function and wins, so the result is bool.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
enum Color { red, green };
struct Odd {};
template <class T> Odd operator==(T, T);
template <class T> Odd operator<(T, T);
static_assert(same<decltype(red == green), bool>);
static_assert(same<decltype(red < green), bool>);
