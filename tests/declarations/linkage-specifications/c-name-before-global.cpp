// tidal-rules: LSP-7
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: in global scope conflicts with declaration with C language linkage
// tidal-error-edg: name linkage conflicts
// tidal-note: Q-0075. The C variable comes first, and the global variable with C++ linkage second. The order does not matter. Clang and EDG reject. GCC accepts.
namespace n { extern "C" int clash; }
int clash;
