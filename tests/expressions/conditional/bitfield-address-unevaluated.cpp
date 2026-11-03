// tidal-rules: COND-9
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field
// tidal-note: The & is in the operand of decltype, which is not evaluated. The operands are unsigned bit-fields.
struct S { unsigned a : 4; unsigned b : 4; };
void f(S& s, bool c) { using P = decltype(&(c ? s.a : s.b)); }
