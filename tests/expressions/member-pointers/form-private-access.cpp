// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
class P { int x; };
auto pm = &P::x;
