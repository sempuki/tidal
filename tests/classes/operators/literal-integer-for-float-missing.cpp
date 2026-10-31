// tidal-rules: OPR-8
// tidal-expect: reject
unsigned long long operator""_u(unsigned long long n) { return n; }
auto x = 1.5_u;
