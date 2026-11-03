// tidal-rules: CLS-23
// tidal-expect: reject
// tidal-error-clang: requires .constexpr. specifier
// tidal-error-gcc: .constexpr. needed for in-class initialization
// tidal-error-edg: cannot have an in-class initializer
// tidal-note: A const static data member of floating-point type that is not constexpr or inline is not one of the members that CLS-3 lists.
struct S { static const double d = 1.5; };
