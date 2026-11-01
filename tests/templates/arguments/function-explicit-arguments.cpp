// tidal-rules: TARG-20
// tidal-expect: accept
// tidal-note: The explicit arguments go to the leading parameters. The last parameter comes from its default.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class R, class T, class U = long> constexpr R convert(T v) { return static_cast<R>(v + U(0)); }
static_assert(same<decltype(convert<int, double>(1.5)), int>);
static_assert(convert<int, double>(1.5) == 1);
static_assert(same<decltype(convert<char>(65)), char>);
