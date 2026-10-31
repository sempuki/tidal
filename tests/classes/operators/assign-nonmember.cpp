// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: must be a non-static member function
// tidal-error-gcc: must be a (non-static )?member function
// tidal-error-edg: must be a member function
struct S {};
S& operator=(S&, int);
