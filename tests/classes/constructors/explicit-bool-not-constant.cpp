// tidal-rules: CTOR-9
// tidal-expect: reject
bool flag = true;
struct S { explicit(flag) S(int) {} };
