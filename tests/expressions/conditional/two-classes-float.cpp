// tidal-rules: COND-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0373. float is a promoted arithmetic type, so two classes that convert to float give float. Clang and GCC accept. EDG gives double and rejects the static_assert.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
struct First { operator float(); };
struct Second { operator float(); };
extern bool c;
First f;
Second s;
static_assert(same<decltype(c ? f : s), float>);
