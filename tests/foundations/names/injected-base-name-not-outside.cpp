// tidal-rules: NAM-36
// tidal-expect: reject
// tidal-error-clang: unknown type name .base.
// tidal-error-gcc: base. does not name a type
// tidal-error-edg: identifier .base. is undefined
namespace lib { struct base { int v = 1; }; }
struct derived : lib::base { };
base copy;
