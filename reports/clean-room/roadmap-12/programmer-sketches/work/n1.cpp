template <class T> struct Ring {
  union Slot { T value; constexpr Slot() {} constexpr ~Slot() {} };
  Slot s[2];
  int n = 0;
  constexpr Ring() = default;
};
constexpr int f() { Ring<int> r; return r.n; }
static_assert(f() == 0);
int main() { return 0; }
