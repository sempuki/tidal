// tidal-rules: THR-3
// tidal-expect: reject
// tidal-error-clang: private destructor
// tidal-error-gcc: D::~D\(\). is private within this context
// tidal-error-edg: D::~D\(\).* is inaccessible
class D { ~D() {} public: D() {} };
void f() { throw D(); }
