// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: virtual. can only appear on non-static member functions
// tidal-error-gcc: declared as a .virtual. field
// tidal-error-edg: virtual. is not allowed
struct S { virtual int x; };
