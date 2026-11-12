struct P { int lo, hi; };
struct S {
  int n = 0;
  constexpr S(const P (&init)[2]) : n(init[1].lo) {}
};
static_assert(S({{1, 2}, {3, 4}}).n == 3);
int main() { return 0; }
