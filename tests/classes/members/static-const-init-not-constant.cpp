// tidal-rules: CLS-20
// tidal-expect: reject
// tidal-error-clang: in-class initializer for static data member is not a constant expression
// tidal-error-gcc: not usable in a constant expression
// tidal-error-edg: must have a constant value
// tidal-note: Gap review 2026-10-31.
int n = 1;
struct S { static const int k = n; };
