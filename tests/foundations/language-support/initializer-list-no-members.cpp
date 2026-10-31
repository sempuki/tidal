// tidal-rules: LSUP-1
// tidal-expect: reject
// tidal-error-clang: layout not recognized|weird std::initializer_list
// tidal-error-gcc: does not match
// tidal-error-edg: too many initializer values|expected constructor
namespace std {
template <class E> class initializer_list {};
}
void take(std::initializer_list<int>);
void use() { take({1, 2}); }
