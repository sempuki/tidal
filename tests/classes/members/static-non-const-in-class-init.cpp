// tidal-rules: CLS-3
// tidal-expect: reject
// tidal-error-clang: non-const static data member must be initialized out of line
// tidal-error-gcc: in-class initialization of non-const static member
// tidal-error-edg: in-class initializer must be const
struct S { static int n = 4; };
