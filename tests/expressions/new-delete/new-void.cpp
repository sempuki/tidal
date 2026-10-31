// tidal-rules: NEW-7
// tidal-expect: reject
// tidal-error-clang: allocation of incomplete type .void.
// tidal-error-gcc: invalid use of .void.
// tidal-error-edg: incomplete type .void. is not allowed
void f() { auto p = new void(); }
