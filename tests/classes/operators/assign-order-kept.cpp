// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct S { S& operator=(int) { return *this; } };
S& left(S& s) { trace = trace * 10 + 2; return s; }
int right() { trace = trace * 10 + 1; return 0; }
int main() { S s; left(s) = right(); return trace == 12 ? 0 : 1; }
