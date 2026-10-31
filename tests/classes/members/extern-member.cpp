// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: storage class specified for a member
// tidal-error-gcc: storage class specified for
// tidal-error-edg: invalid storage class for a class member
// tidal-note: Gap review 2026-10-31.
struct S { extern int v; };
