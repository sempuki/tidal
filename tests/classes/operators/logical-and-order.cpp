// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct B { bool v; };
bool operator&&(B a, B b) { return a.v && b.v; }
bool operator||(B a, B b) { return a.v || b.v; }
B mark(int d) { trace = trace * 10 + d; return {true}; }
int main() { (void)(mark(1) && mark(2)); (void)(mark(3) || mark(4)); return trace == 1234 ? 0 : 1; }
