// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct M { int v = 3; };
struct P { M m; int operator->*(int M::* pm) { return m.*pm; } };
P& left(P& p) { trace = trace * 10 + 1; return p; }
int M::* right() { trace = trace * 10 + 2; return &M::v; }
int main() { P p; int r = left(p)->*right(); return r == 3 && trace == 12 ? 0 : 1; }
