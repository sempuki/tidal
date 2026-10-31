// tidal-rules: CMP-1
// tidal-expect: accept
// tidal-expect-clang-22: reject
// tidal-expect-clang-host: reject
// tidal-error-clang: chained comparison
// tidal-note: Q-0029. Clang 22 rejects a chained comparison by default (-Wparentheses, default error); Clang 18, GCC, and EDG accept it.
static_assert(3 > 2 > 1 == false);
