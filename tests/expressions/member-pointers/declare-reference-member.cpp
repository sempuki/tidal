// tidal-rules: MPTR-8
// tidal-expect: reject
// tidal-error-clang: pointer-to-member to member .r. of reference type
// tidal-error-gcc: cannot create pointer to reference member
// tidal-error-edg: pointer to member of type .int &. is not allowed
struct S { int& r; };
auto pm = &S::r;
