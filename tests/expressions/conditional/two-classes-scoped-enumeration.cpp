// tidal-rules: COND-11
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: operands to .\?:. have different types
// tidal-note: Q-0373. A scoped enumeration has a built-in candidate with two parameters of its type. Clang and EDG accept. GCC has no such candidate and rejects.
template <class X, class Y> constexpr bool same = false;
template <class X> constexpr bool same<X, X> = true;
enum class Mode { on };
struct First { operator Mode(); };
struct Second { operator Mode(); };
extern bool c;
First f;
Second s;
static_assert(same<decltype(c ? f : s), Mode>);
