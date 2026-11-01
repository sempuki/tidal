// tidal-rules: CPT-24
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct Runner { void run(); };
template <class T> constexpr bool adds = requires (T t) { t + t; };
template <class T> constexpr bool kind() { return same<decltype(requires (T t) { t; }), bool>; }
static_assert(adds<int> && !adds<Runner> && kind<int>());
template <bool V> struct Flag {};
template <class T> using Checked = Flag<requires (T t) { t; }>;
Checked<int> flag;
constexpr bool runs = requires (Runner r) { r.run(); };
static_assert(runs && requires { 1 + 1; });
