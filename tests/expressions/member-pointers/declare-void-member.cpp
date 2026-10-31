// tidal-rules: MPTR-8
// tidal-expect: reject
// tidal-error-clang: member pointer to void
// tidal-error-gcc: cannot declare pointer to .void. member
// tidal-error-edg: pointer to member of type void
struct S {};
void S::* pm;
