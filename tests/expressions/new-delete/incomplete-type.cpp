// tidal-rules: NEW-7
// tidal-expect: reject
// tidal-error-clang: allocation of incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: incomplete type .Incomplete. is not allowed
struct Incomplete;
void f() { auto p = new Incomplete; }
