// tidal-rules: LSP-4
// tidal-expect: reject
// tidal-error-clang: conflicting types for
// tidal-error-gcc: conflicting C language linkage declaration
// tidal-error-edg: declaration is incompatible with previous
namespace a { extern "C" void conflict(int); }
namespace b { extern "C" void conflict(double); }
