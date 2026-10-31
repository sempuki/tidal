// tidal-rules: AUTO-21
// tidal-expect: reject
// tidal-error-clang: std::initializer_list was not found
// tidal-error-gcc: requires .#include <initializer_list>
// tidal-error-edg: #include <initializer_list> is needed
auto a = {1, 2};
