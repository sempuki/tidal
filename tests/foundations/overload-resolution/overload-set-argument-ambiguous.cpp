// tidal-rules: OVL-36 OVL-39
// tidal-expect: reject
// tidal-error-clang: call to .g. is ambiguous
// tidal-error-gcc: call of overloaded .g\(<unresolved overloaded function type>\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .g. matches the argument list
// tidal-note: Two pairings match, and their sequences are both the identity.
void f(int);
void f(char);
void g(void (*)(int));
void g(void (*)(char));
void use() { g(f); }
