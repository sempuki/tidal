// tidal-rules: EXS-4
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Q-0095. EDG treats a defaulted == as non-throwing even if a member == is potentially throwing. Clang and GCC do not.
struct M { bool operator==(const M&) const { return true; } };
struct T { M m; bool operator==(const T&) const = default; };
static_assert(!noexcept(T{} == T{}));
