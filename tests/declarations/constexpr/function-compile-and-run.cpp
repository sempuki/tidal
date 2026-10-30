// tidal-rules: CX-3
// tidal-expect: run
constexpr int square(int v) { return v * v; }
static_assert(square(4) == 16);
int main(int argc, char**) { return square(argc) == argc * argc ? 0 : 1; }
