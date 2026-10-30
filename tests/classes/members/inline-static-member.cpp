// tidal-rules: CLS-3
// tidal-expect: run
// tidal-sources: inline-static-member.tu.cpp
struct Config { inline static int level = 3; static constexpr int max = 9; };
const int* other_level();
int main() { return &Config::level == other_level() && Config::max == 9 ? 0 : 1; }
