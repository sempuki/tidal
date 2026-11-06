// The same at run time. g++ and clang++ exit 0. tidal-fe gives result 1.
struct M { int c[1][1]; bool operator==(const M&) const = default; };
int main() { M x{}, y{}; return x == y ? 0 : 1; }
