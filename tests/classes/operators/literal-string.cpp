// tidal-rules: OPR-8
// tidal-expect: run
using Size = decltype(sizeof 0);
Size operator""_len(const char*, Size n) { return n; }
int main() { return "abcd"_len == 4 ? 0 : 1; }
