// tidal-rules: MPTR-8
// tidal-expect: reject
// tidal-error-clang: member pointer to a reference
// tidal-error-gcc: cannot declare pointer to .int&.
// tidal-error-edg: pointer to member of reference type
struct S {};
int& S::* pm;
