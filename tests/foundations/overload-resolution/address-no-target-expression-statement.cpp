// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: reference to overloaded function could not be resolved
// tidal-error-gcc: statement cannot resolve address of overloaded function
// tidal-error-edg: overloaded|cannot|no instance|deduce
// tidal-note: An expression statement gives no target type.
void f(int);
void f(double);
void use() { f; }
