// tidal-rules: UNI-11
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .*U::~U
// tidal-error-edg: U::~U\(\) noexcept.* deleted function
struct S { ~S() {} };
union U { S s; int i; U() : i(0) {} };
void f() { U u; }
