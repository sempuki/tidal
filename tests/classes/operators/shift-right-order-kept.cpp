// tidal-rules: OPR-7
// tidal-expect: run
int trace = 0;
struct In { In& operator>>(int) { return *this; } };
In& source(In& in) { trace = trace * 10 + 1; return in; }
int value() { trace = trace * 10 + 2; return 0; }
int main() { In in; source(in) >> value(); return trace == 12 ? 0 : 1; }
