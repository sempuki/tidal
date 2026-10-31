// tidal-rules: VOL-4
// tidal-expect: reject
struct S { int v; };
volatile S a{};
S b = a;
