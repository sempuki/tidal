// tidal-rules: EXS-1
// tidal-expect: reject
// tidal-error-clang: noexcept specifier argument is not a constant expression
// tidal-error-gcc: not usable in a constant expression
// tidal-error-edg: must have a constant value
bool flag = true;
void f() noexcept(flag);
