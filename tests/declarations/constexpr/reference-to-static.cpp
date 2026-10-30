// tidal-rules: CX-2
// tidal-expect: accept
int global = 1;
constexpr int& r = global;
constexpr const int* p = &global;
