// tidal-rules: DCL-7
// tidal-expect: reject
// tidal-error-clang: non-member function cannot have .const. qualifier
// tidal-error-gcc: non-member function .* cannot have cv-qualifier
// tidal-error-edg: type qualifier is not allowed on a nonmember function
void f() const;
