// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .*~S
// tidal-error-edg: ~S\(\) noexcept.* deleted function
struct S { int v; ~S() = delete; };
S make();
S* p = new S(make());
