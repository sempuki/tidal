// tidal-rules: TYN-9
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: non-C-compatible type given name for linkage purposes
// tidal-error-gcc: non-C-compatible type given name for linkage purposes
// tidal-error-edg: non-C-like type given name for linkage purposes
// tidal-note: Q-0067. The unnamed class has a base. GCC 14 accepts it. GCC 16, Clang, and EDG reject it.
struct Base {};
typedef struct : Base { int v; } Derived;
int main() { Derived d{}; return d.v; }
