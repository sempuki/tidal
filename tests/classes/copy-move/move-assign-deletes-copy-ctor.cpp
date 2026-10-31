// tidal-rules: CPY-4
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct S { S() {} S& operator=(S&&) { return *this; } };
S a;
S b(a);
