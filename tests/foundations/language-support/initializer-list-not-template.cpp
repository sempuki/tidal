// tidal-rules: LSUP-1
// tidal-expect: reject
// tidal-error-clang: must be a class template
// tidal-error-gcc: does not match
// tidal-error-edg: is needed prior to a use
namespace std {
class initializer_list {};
}
auto values = {1, 2};
