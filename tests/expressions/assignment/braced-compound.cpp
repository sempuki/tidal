// tidal-rules: ASN-6
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-note: Q-0032. Clang accepts a braced list as the right operand of a compound assignment to a scalar; GCC and EDG reject it.
void f() { int x = 0; x += {1}; }
