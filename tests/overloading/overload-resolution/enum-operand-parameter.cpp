// tidal-rules: OVL-8
// tidal-expect: accept
// tidal-note: The first parameter is the enumeration of the first operand, so the operator is a candidate. The scoped enumeration has no built-in +.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
enum class Mode { on };
struct Wrap { Wrap(Mode); };
One operator+(Mode, Wrap);
static_assert(same<decltype(Mode::on + Mode::on), One>);
