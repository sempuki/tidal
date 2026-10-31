// tidal-rules: NS-14
// tidal-expect: reject
// tidal-error-clang: redefinition of .ln. as different kind of symbol
// tidal-error-gcc: namespace alias .ln. not allowed here
// tidal-error-edg: ln. has already been declared in the current scope
// tidal-note: Gap review 2026-10-31.
namespace long_name { int a; }
namespace ln = long_name;
namespace ln { int b; }
