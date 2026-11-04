// tidal-rules: OVL-20
// tidal-expect: reject
// tidal-error-clang: cannot deduce return type
// tidal-error-gcc: unable to deduce .auto. from .f.
// tidal-error-edg: overloaded|cannot|no instance|deduce
// tidal-note: A deduced return type gives no target type, and the set has two functions.
void f(int);
void f(double);
auto get() { return f; }
