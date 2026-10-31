// tidal-rules: EXS-2
// tidal-expect: reject
// tidal-error-clang: exception specification in declaration does not match
// tidal-error-gcc: different exception specifier
// tidal-error-edg: exception specification is incompatible
void f();
void f() noexcept;
