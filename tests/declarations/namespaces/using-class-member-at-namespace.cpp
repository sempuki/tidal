// tidal-rules: NS-9
// tidal-expect: reject
// tidal-error-clang: using declaration cannot refer to class member
// tidal-error-gcc: using-declaration for member at non-class scope
// tidal-error-edg: class-qualified name is not allowed
// tidal-note: Gap review 2026-10-31.
struct S { static int n; };
using S::n;
