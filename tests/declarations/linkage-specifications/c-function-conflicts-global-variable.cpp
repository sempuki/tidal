// tidal-rules: LSP-7
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: C language linkage conflicts with declaration in global scope|in global scope conflicts with declaration with C language linkage
// tidal-error-edg: declaration is incompatible with previous "clash"
// tidal-note: Q-0075. A C function also must not have the spelling of a global variable with C++ linkage, in either order. Clang and EDG reject. GCC accepts.
int clash;
namespace n { extern "C" int clash(); }
namespace m { extern "C" int other(); }
int other;
