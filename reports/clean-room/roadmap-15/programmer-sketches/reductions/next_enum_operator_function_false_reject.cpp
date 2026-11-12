enum class Flag { none = 0, a = 1, b = 2 };
constexpr Flag operator&(Flag x, Flag y) { return static_cast<Flag>(static_cast<int>(x) & static_cast<int>(y)); }
int main() { return (Flag::a & Flag::b) == Flag::none ? 0 : 1; }
