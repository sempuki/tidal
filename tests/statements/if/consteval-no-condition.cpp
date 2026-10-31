// tidal-rules: IF-20
// tidal-expect: reject
// tidal-error-clang: expected \{ after consteval
// tidal-error-gcc: if consteval. requires compound statement
// tidal-error-edg: require braced dependent statements
constexpr int f() {
  if consteval (true) { return 1; }
  return 2;
}
