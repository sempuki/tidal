// tidal-rules: MPTR-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int S::\*
// tidal-error-gcc: invalid operands of types .int S::\*. and .int S::\*. to binary .operator<=>
// tidal-error-edg: for built-in operator<=>
struct S { int a; int b; };
bool less = (&S::a <=> &S::b) < 0;
