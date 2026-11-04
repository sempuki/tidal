// tidal-rules: OVL-36 OVL-25
// tidal-expect: reject
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .f. matches the argument list
// tidal-note: The parameter filled by the default argument takes no part, so the two functions tie on the one argument.
void f(int);
void f(int, int = 0);
void use() { f(1); }
