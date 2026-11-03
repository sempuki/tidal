// tidal-rules: CLS-20
// tidal-expect: reject
// tidal-error-clang: in-class initializer for static data member is not a constant expression
// tidal-error-gcc: not usable in a constant expression
// tidal-error-edg: must have a constant value
// tidal-note: The member has enumeration type, and its initializer reads a variable that is not const.
enum Mode { slow, fast };
Mode current = fast;
struct S { static const Mode m = current; };
