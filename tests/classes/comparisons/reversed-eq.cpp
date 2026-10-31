// tidal-rules: DCMP-6
// tidal-expect: run
struct S { int v; bool operator==(int o) const { return v == o; } };
int main() { S s{3}; return 3 == s && s == 3 ? 0 : 1; }
