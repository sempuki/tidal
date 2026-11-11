// A consteval constructor checks its argument in a constant expression.
struct Port { int n; consteval Port(int v) : n(v) { if (v < 1 || v > 65535) throw "bad port"; } };
int open(Port p) { return p.n; }
int main() { return open(8080) - 8080; }
