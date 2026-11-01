// tidal-rules: LSP-1 LSP-6
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: is incompatible with parameter of type
// tidal-note: Q-0074. The parameter of run has a pointer to a C function type. EDG rejects a C++ function as the argument. Clang and GCC accept it.
extern "C" void run(int (*g)(int)) { g(1); }
int plain(int v) { return v; }         // C++ language linkage
int main() { run(plain); }
