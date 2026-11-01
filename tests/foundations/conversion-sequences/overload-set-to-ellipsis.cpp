// tidal-rules: ICS-6 ICS-47 OVL-36
// tidal-expect: reject
// tidal-error-clang: reference to overloaded function could not be resolved
// tidal-error-gcc: overloaded function with no contextual type information
// tidal-error-edg: overloaded|no instance|cannot
// tidal-note: The set has two functions, and the ellipsis gives no target type, so g is not viable.
void f(int);
void f(double);
void g(...);
void use() { g(f); }
