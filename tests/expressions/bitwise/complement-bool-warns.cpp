// tidal-rules: BIT-7
// tidal-expect: reject
// tidal-error-clang: bitwise negation of a boolean expression
// tidal-error-gcc: on an expression of type .bool.
// tidal-flags: -Werror -Wbool-operation
int f(bool b) {
  return ~b;
}
