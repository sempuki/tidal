// tidal-rules: ENUM-14
// tidal-expect: reject
// tidal-error-clang: redefinition of enumerator
// tidal-error-gcc: redefinition of .A
// tidal-error-edg: A. has already been declared
enum E { A, A };
