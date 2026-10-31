// tidal-rules: VOL-2
// tidal-expect: accept
struct S { int v; };
volatile S s{};
volatile int* p = &s.v;
