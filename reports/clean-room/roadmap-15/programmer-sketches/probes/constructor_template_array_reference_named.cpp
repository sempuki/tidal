struct P { int lo, hi; };
struct S {
  int n = 0;
  template <decltype(sizeof 0) M>
  constexpr S(const P (&init)[M]) : n(static_cast<int>(M)) {}
};
constexpr P ps[] = {{1, 2}, {3, 4}};
static_assert(S(ps).n == 2);
int main() { return 0; }
