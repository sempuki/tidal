// tidal-rules: CMP-14 PRIM-27
// tidal-expect: accept
// tidal-expect-clang-22: reject
// tidal-expect-clang-host: reject
// tidal-error-clang: comparison in fold expression
// tidal-flags: -Werror -Wparentheses
// tidal-note: Q-0029. Each nested part of a fold acts as if in parentheses, so a fold is not a chained comparison. Clang 22 rejects a fold over < by default. GCC and Clang 18 give no warning.
template <class... T> bool ordered(T... p) { return (... < p); }
bool g() { return ordered(1, 2, 3); }
