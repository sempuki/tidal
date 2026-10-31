// tidal-rules: DCMP-6
// tidal-expect: run
struct S { int v; bool operator==(int o) const { return v == o; } };
int main() { S s{3}; return s != 4 && 4 != s ? 0 : 1; }
