// tidal-rules: ILC-1
// tidal-expect: reject
struct S { int x; S(int v) : x(v) {} };
S s{.x = 1};
