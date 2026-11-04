// tidal-rules: OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .f.
// tidal-error-gcc: no matching function for call to .f\(Empty\).
// tidal-error-edg: no instance of overloaded function .f. matches the argument list
// tidal-note: Both candidates have the right count, but no sequence converts Empty.
struct Empty {};
void f(int);
void f(double);
void use() { f(Empty{}); }
