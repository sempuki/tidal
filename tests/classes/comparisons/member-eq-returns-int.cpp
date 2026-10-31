// tidal-rules: DCMP-2
// tidal-expect: run
struct M { int v; int operator==(const M& o) const { return v == o.v; } };
struct S { M m; bool operator==(const S&) const = default; };
int main() { return S{{1}} == S{{1}} && !(S{{1}} == S{{2}}) ? 0 : 1; }
