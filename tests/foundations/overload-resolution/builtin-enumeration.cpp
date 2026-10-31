// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: A scoped enumeration has comparison candidates and no arithmetic candidates.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
enum class Mode { a, b };
struct ToMode { operator Mode() const; };
static_assert(same<decltype(ToMode{} < ToMode{}), bool>);
static_assert(same<decltype(ToMode{} == Mode::a), bool>);
