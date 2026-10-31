// tidal-rules: CLS-27
// tidal-expect: reject
// tidal-error-clang: static member function cannot have .const. qualifier
// tidal-error-gcc: static member function .* cannot have cv-qualifier
// tidal-error-edg: type qualifier is not allowed on a static member function
struct S { static void f() const; };
