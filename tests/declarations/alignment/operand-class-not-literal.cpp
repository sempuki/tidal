// tidal-rules: ALN-8
// tidal-expect: reject
// tidal-error-clang: requires integer constant
// tidal-error-gcc: non-literal type
// tidal-error-edg: must have a constant value
// tidal-note: W has a user-provided destructor, so it is not a literal type, and a temporary of W cannot appear in a constant. The parentheses make the operand an expression.
struct W {
  ~W() {}
  constexpr operator int() const { return 8; }
};
struct alignas((W{})) Block { int v; };
