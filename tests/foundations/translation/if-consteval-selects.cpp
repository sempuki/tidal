// tidal-rules: TR-5
// tidal-expect: run
constexpr int where() { if consteval { return 1; } else { return 2; } }
static_assert(where() == 1);
int main() { return where() == 2 ? 0 : 1; }
