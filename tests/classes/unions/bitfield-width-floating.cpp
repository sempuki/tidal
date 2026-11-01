// tidal-rules: BF-2
// tidal-expect: reject
// tidal-error-gcc: has non-integral type
// tidal-error-clang: must have integral or unscoped enumeration type
// tidal-error-edg: instead of the required integral or enum type
// tidal-note: The width of a bit-field must have integral or unscoped enumeration type. A floating-point constant is not valid, also with an integer value.
struct Flags { int mode : 3.0; };
