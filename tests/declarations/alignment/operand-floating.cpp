// tidal-rules: ALN-8
// tidal-expect: reject
// tidal-error-gcc: non-integral type
// tidal-error-clang: requires integer constant
// tidal-error-edg: instead of the required integral or enum type
// tidal-note: The operand of alignas must have integral or unscoped enumeration type. A floating-point constant is not valid, also when its value is a power of two.
struct alignas(8.0) Block { int v; };
