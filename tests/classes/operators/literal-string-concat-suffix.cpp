// tidal-rules: OPR-8
// tidal-expect: run
using Size = decltype(sizeof 0);
Size operator""_n(const char*, Size n) { return n; }
int main() { return "ab" "cd"_n == 4 && "ab"_n "cd"_n == 4 ? 0 : 1; }
