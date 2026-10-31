// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: has private destructor
// tidal-error-gcc: private within this context
// tidal-error-edg: is inaccessible
class S { ~S() {} };
void f() { S s; }
