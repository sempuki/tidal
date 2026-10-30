// tidal-rules: DTOR-8
// tidal-expect: reject
struct S { ~S() {} };
auto p = &S::~S;
