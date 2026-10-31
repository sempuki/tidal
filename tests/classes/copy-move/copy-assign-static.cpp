// tidal-rules: CPY-16
// tidal-expect: reject
// tidal-error-clang: cannot be a static member function
// tidal-error-gcc: must be a non-static member function
// tidal-error-edg: may not be a static member function
struct S { static S& operator=(const S&); };
