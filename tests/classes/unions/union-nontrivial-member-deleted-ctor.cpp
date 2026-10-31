// tidal-rules: UNI-3
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted default constructor of .U
// tidal-error-gcc: use of deleted function .*U::U\(\)
// tidal-error-edg: default constructor of .U. cannot be referenced
struct S { S() {} ~S() {} };
union U { S s; int i; };
U u;
