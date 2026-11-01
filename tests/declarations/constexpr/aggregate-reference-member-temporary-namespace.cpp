// tidal-rules: CX-2
// tidal-expect: accept
struct Holder { const int& n; };
constexpr Holder kept{4};
static_assert(kept.n == 4);
int main() {}
