// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: taking the address of a destructor
// tidal-error-gcc: taking address of destructor
// tidal-error-edg: may not have its address taken
struct S { ~S() {} };
auto p = &S::~S;
