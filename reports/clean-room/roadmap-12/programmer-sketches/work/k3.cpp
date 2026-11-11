union Slot { int value; constexpr Slot() {} };
constexpr int f() { Slot s; s.value = 1; return s.value; }
static_assert(f() == 1);
int main() { return 0; }
