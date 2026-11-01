// tidal-rules: DEP-30
// tidal-expect: reject
// tidal-error-clang: undeclared identifier 'helper'
// tidal-error-gcc: declaration of .helper. must be available
// tidal-error-edg: identifier "helper" is undefined
template <class T> int use(T) { return helper(1); }
