// tidal-rules: CPY-4
// tidal-expect: reject
struct S { S() {} S(S&&) {} };
S a;
S b(a);
