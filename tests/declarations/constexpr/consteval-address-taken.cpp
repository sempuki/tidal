// tidal-rules: CEV-2
// tidal-expect: reject
consteval int one() { return 1; }
int (*p)() = one;
