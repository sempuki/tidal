// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct A { int operator[](int i) const { return i; } };
A& get(A& a) { trace = trace * 10 + 1; return a; }
int idx() { trace = trace * 10 + 2; return 0; }
int main() { A a; get(a)[idx()]; return trace == 12 ? 0 : 1; }
