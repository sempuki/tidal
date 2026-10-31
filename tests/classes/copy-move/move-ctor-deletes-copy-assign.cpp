// tidal-rules: CPY-4
// tidal-expect: reject
// tidal-error-clang: copy assignment operator is implicitly deleted
// tidal-error-gcc: use of deleted function .*operator=
// tidal-error-edg: operator=.* deleted function
struct S { S() {} S(S&&) {} };
void f(S& a, S& b) { a = b; }
