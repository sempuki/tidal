// tidal-rules: LSP-1 LSP-6
// tidal-allows-edg: LSP-6
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: is incompatible with parameter of type
// tidal-note: Q-0074. The alias declared in the C block names a function type with C language linkage. EDG records the linkage in the type and rejects a C++ function as the argument. Clang and GCC do not record it.
extern "C" {
using callback = int(int);             // a C function type
int apply(callback* f, int v) { return f(v); }
}
int twice(int v) { return 2 * v; }     // C++ language linkage
int main() { return apply(twice, 2) == 4 ? 0 : 1; }
