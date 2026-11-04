// tidal-rules: OVL-36 OVL-34
// tidal-expect: reject
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int, int\). is ambiguous
// tidal-error-edg: more than one instance of overloaded function .f. matches the argument list
// tidal-note: f(int, long) and f(long, int) are each better than f(long, long), but neither is better than the other.
void f(int, long);
void f(long, long);
void f(long, int);
void use() { f(1, 1); }
