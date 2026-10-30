// tidal-rules: CEV-1
// tidal-expect: reject
consteval int square(int v) { return v * v; }
int f(int x) { return square(x); }
