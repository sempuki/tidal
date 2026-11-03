// Probe: virtual before constexpr, and constexpr on a pure function.
struct P { virtual constexpr int f() const = 0; virtual ~P() = default; };
struct Q : P { constexpr int f() const override { return 3; } };
static_assert(Q{}.f() == 3);
int main() { return 0; }
