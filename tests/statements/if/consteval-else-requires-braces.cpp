// tidal-rules: IF-20
// tidal-expect: reject
// tidal-error-clang: expected \{ after else
// tidal-error-gcc: if consteval. requires compound statement
// tidal-error-edg: require braced dependent statements
constexpr int f() {
  if consteval { return 1; } else return 2;
}
