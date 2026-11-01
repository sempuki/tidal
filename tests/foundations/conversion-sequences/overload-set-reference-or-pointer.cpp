// tidal-rules: ICS-47 OVL-36
// tidal-expect: reject
// tidal-error-clang: call to .g. is ambiguous
// tidal-error-gcc: call of overloaded .g\(<unresolved overloaded function type>\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .g. matches
// tidal-note: Both candidates select f(int). The reference binds with the identity, and the pointer needs the function to pointer conversion, which is an lvalue transformation. Neither sequence is better.
void f(int);
void f(double);
void g(void (&)(int));
void g(void (*)(int));
void use() { g(f); }
