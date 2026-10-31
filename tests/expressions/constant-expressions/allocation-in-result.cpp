// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: refers to a result of .operator new.
constexpr int* make() { return new int(1); }
constexpr int* p = make();
