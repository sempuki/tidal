// Probe: a constexpr function that makes a local object of a class with a virtual base.
struct L { int n = 0; };
struct X : virtual L {};
struct Y : virtual L {};
struct Z : X, Y {};
constexpr int f() { Z z; return z.n; }
int main() { return f(); }
