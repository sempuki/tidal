// tidal-rules: CPY-21
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct S { S() {} S(S&&) {} };
S a;
S b(a);
