// tidal-rules: CTOR-9
// tidal-expect: reject
// tidal-error-clang: explicit specifier argument is not a constant expression
// tidal-error-gcc: not usable in a constant expression
// tidal-error-edg: must have a constant value
bool flag = true;
struct S { explicit(flag) S(int) {} };
