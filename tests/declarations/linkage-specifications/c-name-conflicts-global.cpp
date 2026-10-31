// tidal-rules: LSP-4
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: C language linkage conflicts with declaration in global scope
// tidal-error-edg: name linkage conflicts
// tidal-note: Q-0075. GCC accepts a C language linkage variable with the name of a global variable. Clang and EDG reject it.
int clash;
namespace n { extern "C" int clash; }
