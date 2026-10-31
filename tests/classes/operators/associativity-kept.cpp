// tidal-rules: OPR-11
// tidal-expect: run
struct N { int v; };
N operator-(N a, N b) { return {a.v - b.v}; }
int main() { N a{10}, b{3}, c{2}; return (a - b - c).v == 5 ? 0 : 1; }
