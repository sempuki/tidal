// tidal-rules: ASN-14
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: brace-enclosed initializer list
// tidal-error-edg: brace-enclosed list is not allowed
// tidal-note: Q-0032. Clang accepts a braced list as the right operand of a compound assignment to a scalar; GCC and EDG reject it.
void f() { int x = 0; x += {1}; }
