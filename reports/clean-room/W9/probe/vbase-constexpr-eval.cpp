// Probe: the same function used in a static_assert.
struct L { int n = 0; };
struct X : virtual L {};
struct Y : virtual L {};
struct Z : X, Y {};
constexpr int f() { Z z; return z.n; }
static_assert(f() == 0);
int main() { return 0; }
