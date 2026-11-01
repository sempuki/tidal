// tidal-rules: UN-18
// tidal-expect: accept
struct Risky { ~Risky() noexcept(false) {} };
struct Calm { ~Calm() {} };
static_assert(!noexcept(Risky()));
static_assert(noexcept(Calm()));
int main() {}
