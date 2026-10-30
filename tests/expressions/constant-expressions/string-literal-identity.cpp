// tidal-rules: CE-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-note: Q-0033. Clang rejects comparing addresses of two string literals during constant evaluation; GCC and EDG fold it.
constexpr bool same_literal() { return "abc" == "abc"; }
static_assert(same_literal() || !same_literal());
