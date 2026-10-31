// tidal-rules: OVL-36 OVL-25
// tidal-expect: reject
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int, int\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .f. matches the argument list
// tidal-note: Each function is better on one argument and worse on the other, so neither is better.
void f(int, long);
void f(long, int);
void use() { f(1, 1); }
