// tidal-rules: VIRT-9
// tidal-expect: reject
// tidal-error-clang: friend declaration cannot have a pure-specifier
// tidal-error-gcc: initializer specified for friend function
// tidal-error-edg: allowed only on virtual functions
struct S { friend void f() = 0; };
