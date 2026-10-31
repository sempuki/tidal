// tidal-rules: NAM-21
// tidal-expect: accept
namespace n { extern int total; constexpr int base = 4; }
int n::total = base * 2;
