union Slot { int value; constexpr Slot() {} };
constexpr int f() { Slot s[2]; s[0].value = 1; return s[0].value; }
static_assert(f() == 1);
int main() { return 0; }
