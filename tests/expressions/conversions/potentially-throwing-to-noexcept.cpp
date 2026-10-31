// tidal-rules: CONV-10
// tidal-expect: reject
// tidal-error-clang: different exception specifications
// tidal-error-gcc: invalid conversion from .void \(\*\)\(\). to .void \(\*\)\(\) noexcept.
// tidal-error-edg: cannot be used to initialize an entity of type .void \(\*\)\(\) noexcept
void g();
void (*q)() noexcept = g;
