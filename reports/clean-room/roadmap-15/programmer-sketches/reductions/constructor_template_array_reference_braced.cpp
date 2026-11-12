struct P { int lo, hi; };
struct S {
  int n = 0;
  template <decltype(sizeof 0) M>
  constexpr S(const P (&init)[M]) : n(static_cast<int>(M)) {}
};
static_assert(S({{1, 2}, {3, 4}}).n == 2);
int main() { return 0; }
