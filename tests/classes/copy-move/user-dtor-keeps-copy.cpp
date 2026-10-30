// tidal-rules: CPY-4
// tidal-expect: accept
struct S { ~S() {} };
S a;
S b(a);
