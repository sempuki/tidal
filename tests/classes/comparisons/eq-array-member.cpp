// tidal-rules: DCMP-1
// tidal-expect: run
struct S { int a[3]; bool operator==(const S&) const = default; };
int main() { S x{{1, 2, 3}}, y{{1, 2, 3}}, z{{1, 2, 4}}; return x == y && !(x == z) ? 0 : 1; }
