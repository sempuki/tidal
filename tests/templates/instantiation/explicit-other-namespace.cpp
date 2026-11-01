// tidal-rules: INST-21
// tidal-expect: reject
// tidal-error-clang: not in a namespace enclosing 'lib'
// tidal-error-gcc: not in a namespace surrounding .lib.
// tidal-error-edg: cannot be explicitly instantiated in the current scope
namespace lib { template <class T> int get(T) { return 1; } }
namespace other { template int lib::get(int); }
