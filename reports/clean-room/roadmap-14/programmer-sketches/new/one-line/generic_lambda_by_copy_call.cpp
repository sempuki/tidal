struct Twice { constexpr int operator()(int x) const { return 2 * x; } };
constexpr int apply() { Twice t; auto g = [t](auto x) { return t(x); }; return g(3) + g(4); }
static_assert(apply() == 14);
int main() { return apply() - 14; }
