// tidal-rules: CTOR-9
// tidal-expect: accept
struct S { explicit S(int) {} };
S a(1);
S b{1};
S c = S(1);
