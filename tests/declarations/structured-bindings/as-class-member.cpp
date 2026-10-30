// tidal-rules: SB-7
// tidal-expect: reject
int a[2];
struct S { auto [x, y] = a; };
