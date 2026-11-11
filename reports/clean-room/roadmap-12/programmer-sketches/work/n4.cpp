struct Ring {
  union Slot { int value; constexpr Slot() {} constexpr ~Slot() {} };
  Slot s[2];
  int n = 0;
  constexpr Ring() = default;
  constexpr Ring(const Ring& o) : n(0) { n = o.n; }
  constexpr Ring& operator=(Ring o) { n = o.n; return *this; }
};
constexpr int f() { Ring r; r = Ring(); return r.n; }
static_assert(f() == 0);
int main() { return 0; }
