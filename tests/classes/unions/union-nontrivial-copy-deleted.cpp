// tidal-rules: UNI-11
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted copy constructor of .U
// tidal-error-gcc: use of deleted function .*U::U\(const U&\)
// tidal-error-edg: U::U\(const U &\).* deleted function
struct S { S() {} S(const S&) {} };
union U { S s; int i; U() : i(0) {} };
U a;
U b(a);
