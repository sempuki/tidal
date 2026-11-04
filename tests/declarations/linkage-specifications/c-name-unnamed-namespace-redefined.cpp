// tidal-rules: LSP-4 FN-14
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: redefinition of .code
// tidal-error-gcc: symbol .code. is already defined
// tidal-note: Q-0369. The C function in the unnamed namespace and the C function in b are one entity, so the second definition is not valid. Clang rejects the source. GCC accepts the source and fails at assembly, with one symbol for the two definitions. EDG gives the name in the unnamed namespace internal linkage and no language linkage, and accepts two functions.
namespace { extern "C" int code() { return 3; } }
namespace b { extern "C" int code() { return 4; } }
