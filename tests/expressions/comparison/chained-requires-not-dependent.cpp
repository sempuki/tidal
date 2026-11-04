// tidal-rules: CMP-1
// tidal-expect: accept
// tidal-expect-clang-22: reject
// tidal-expect-clang-host: reject
// tidal-error-clang: chained comparison
// tidal-note: Q-0029. Clang 22 rejects a chained comparison in a requires expression that depends on no template parameter, as it does elsewhere. GCC gives no warning there, even under -Wall.
constexpr bool ok = requires (int a, int b, int c) { a < b < c; };
