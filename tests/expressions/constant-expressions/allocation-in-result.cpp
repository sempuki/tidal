// tidal-rules: CE-7
// tidal-expect: reject
constexpr int* make() { return new int(1); }
constexpr int* p = make();
