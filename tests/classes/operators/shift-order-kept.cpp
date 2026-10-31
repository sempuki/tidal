// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct Out { Out& operator<<(int d) { trace = trace * 10 + d; return *this; } };
int mark(int d) { return d; }
int seq = 0;
int next() { return ++seq; }
int main() { Out o; o << next() << next() << next(); return trace == 123 ? 0 : 1; }
