// tidal-rules: OPR-13
// tidal-expect: reject
// tidal-error-clang: cannot decrement value of type .C
// tidal-error-gcc: no match for .operator--. .operand type is .C.
// tidal-error-edg: no operator .--. matches
// tidal-note: The class declares only the postfix --, and the expression uses the prefix form.
struct C { C operator--(int); };
void f(C c) { --c; }
