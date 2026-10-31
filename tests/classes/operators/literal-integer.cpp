// tidal-rules: OPR-8
// tidal-expect: run
struct Km { unsigned long long v; };
Km operator""_km(unsigned long long n) { return {n}; }
int main() { Km d = 12_km; return d.v == 12 ? 0 : 1; }
