// tidal-rules: CTOR-9
// tidal-expect: accept
struct S { explicit S(int) {} };
S s = static_cast<S>(1);
