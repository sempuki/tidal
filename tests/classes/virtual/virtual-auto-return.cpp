// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: deduced return type cannot be virtual
// tidal-error-gcc: virtual function cannot have deduced return type
// tidal-error-edg: virtual function cannot have a deduced return type
struct S { virtual auto f() { return 1; } };
