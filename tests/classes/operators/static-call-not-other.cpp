// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: cannot be a static member function
// tidal-error-gcc: must be either a non-static member function or a non-member function
// tidal-error-edg: may not be a static member function
struct S { static int operator+(int); };
