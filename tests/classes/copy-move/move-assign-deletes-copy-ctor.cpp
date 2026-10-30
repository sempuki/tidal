// tidal-rules: CPY-4
// tidal-expect: reject
struct S { S() {} S& operator=(S&&) { return *this; } };
S a;
S b(a);
