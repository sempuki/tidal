// Defaulted == on a class with a two-dimensional array member.
// g++ and clang++ accept, and the program exits 0.
// tidal-fe: rejected [SA-1] at line 6 (x == y is false).
struct M { int c[1][1]; bool operator==(const M&) const = default; };
constexpr M x{}, y{};
static_assert(x == y);
int main() {}
