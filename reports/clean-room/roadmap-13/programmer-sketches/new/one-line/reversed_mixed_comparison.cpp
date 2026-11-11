// A == with an int operand, used with the int on the left (rewritten).
struct Meters {
  int v;
  constexpr bool operator==(int o) const { return v == o; }
};
static_assert(5 == Meters{5});
static_assert(Meters{5} != 6);
static_assert(6 != Meters{5});
int main() { return (3 == Meters{3}) ? 0 : 1; }
