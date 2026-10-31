// tidal-rules: DCMP-1
// tidal-expect: run
int calls = 0;
struct M { int v; bool operator==(const M& o) const { ++calls; return v == o.v; } };
struct S { M a; M b; bool operator==(const S&) const = default; };
int main() { S x{{1}, {2}}, y{{9}, {2}}; bool r = x == y; return !r && calls == 1 ? 0 : 1; }
