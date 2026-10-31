// tidal-rules: OPR-7
// tidal-expect: run
int calls = 0;
struct B { bool v; };
bool operator&&(B a, B b) { return a.v && b.v; }
B make(bool v) { ++calls; return {v}; }
int main() { bool r = make(false) && make(true); return !r && calls == 2 ? 0 : 1; }
