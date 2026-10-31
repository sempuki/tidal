// tidal-rules: OPR-11
// tidal-expect: run
struct N { int v; };
N operator+(N a, N b) { return {a.v + b.v}; }
N operator*(N a, N b) { return {a.v * b.v}; }
int main() { N a{2}, b{3}, c{4}; return (a + b * c).v == 14 ? 0 : 1; }
