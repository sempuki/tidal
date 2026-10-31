// tidal-rules: CEV-1
// tidal-expect: reject
consteval int sq(int v) { return v * v; }
constexpr int wrap(int v) { return sq(v); }
int main(int argc, char**) { return wrap(argc); }
