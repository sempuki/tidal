// tidal-rules: NAM-22
// tidal-expect: reject
// tidal-error-clang: unknown type name .item.
// tidal-error-gcc: item. does not name a type
// tidal-error-edg: identifier .item. is undefined
namespace n { struct item { }; item make(); }
item n::make() { return {}; }
