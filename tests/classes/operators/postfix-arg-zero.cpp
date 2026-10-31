// tidal-rules: OPR-4
// tidal-expect: run
int seen = -1;
struct C { C operator++(int n) { seen = n; return *this; } };
int main() { C c; c++; c.operator++(7); return seen == 7 ? 0 : 1; }
