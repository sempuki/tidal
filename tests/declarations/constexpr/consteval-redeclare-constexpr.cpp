// tidal-rules: CEV-3
// tidal-expect: reject
consteval int f();
constexpr int f() { return 1; }
