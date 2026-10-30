// tidal-rules: DTOR-6
// tidal-expect: reject
struct S { ~S() {} };
auto p = &S::~S;
