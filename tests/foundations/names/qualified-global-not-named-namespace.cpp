// tidal-rules: NAM-14
// tidal-expect: reject
// tidal-error-clang: no member named .value. in the global namespace
// tidal-error-gcc: ::value. has not been declared
// tidal-error-edg: the global scope has no .value.
namespace n { int value = 1; int read() { return ::value; } }
