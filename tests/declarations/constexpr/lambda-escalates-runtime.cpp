// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
consteval int twice(int v) { return v * 2; }
auto call = [](int v) { return twice(v); };
int g(int x) { return call(x); }
