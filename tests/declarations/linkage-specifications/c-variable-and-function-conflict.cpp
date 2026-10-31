// tidal-rules: LSP-4
// tidal-expect: reject
// tidal-error-clang: as different kind of symbol
// tidal-error-gcc: conflicting C language linkage declaration
// tidal-error-edg: declaration is incompatible with previous
extern "C" int item;
namespace n { extern "C" void item(); }
