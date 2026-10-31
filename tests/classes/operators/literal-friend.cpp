// tidal-rules: OPR-10
// tidal-expect: run
struct Km { unsigned long long v;
  friend Km operator""_km(unsigned long long n) { return {n}; }
  friend Km operator""_mi(unsigned long long n);
};
Km operator""_mi(unsigned long long n) { return {n * 16 / 10}; }
int main() { return (5_mi).v == 8 ? 0 : 1; }
