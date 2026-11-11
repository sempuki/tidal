struct Ring {
  union Slot { int value; constexpr Slot() {} constexpr ~Slot() {} };
  Slot s[2];
  int n = 0;
  constexpr Ring() = default;
  constexpr Ring(Ring&& o) : n(0) { n = o.n; }
};
constexpr int f() { Ring r; Ring m = static_cast<Ring&&>(r); return m.n; }
static_assert(f() == 0);
int main() { return 0; }
