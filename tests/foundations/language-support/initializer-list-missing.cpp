// tidal-rules: LSUP-1
// tidal-expect: reject
// tidal-error-clang: std::initializer_list was not found
// tidal-error-gcc: requires .#include <initializer_list>.
// tidal-error-edg: is needed prior to a use
auto values = {1, 2};
