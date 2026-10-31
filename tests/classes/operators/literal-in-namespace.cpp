// tidal-rules: OPR-8
// tidal-expect: run
namespace units { constexpr int operator""_m(unsigned long long n) { return int(n); } }
using namespace units;
int main() { return 3_m == 3 ? 0 : 1; }
