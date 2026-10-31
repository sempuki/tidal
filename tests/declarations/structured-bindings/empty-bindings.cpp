// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: group to be empty
// tidal-error-gcc: empty structured binding declaration
struct E {};
E e;
auto [] = e;
