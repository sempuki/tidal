// tidal-rules: CEV-1
// tidal-expect: reject
// tidal-error-clang: call to immediate function .*operator\(\). is not a constant expression
// tidal-error-gcc: call to consteval function .call\..lambda
// tidal-error-edg: call to consteval function .lambda
// tidal-note: Gap review 2026-10-31.
consteval int twice(int v) { return v * 2; }
auto call = [](int v) { return twice(v); };
int g(int x) { return call(x); }
