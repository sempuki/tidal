union Slot { int value; constexpr Slot() {} };
struct Ring { Slot s[2]; };
constexpr int f() { Ring r; r.s[0].value = 1; return r.s[0].value; }
static_assert(f() == 1);
int main() { return 0; }
