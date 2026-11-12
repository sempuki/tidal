struct P { int lo, hi; };
template <decltype(sizeof 0) M>
constexpr int count(const P (&)[M]) { return static_cast<int>(M); }
static_assert(count({{1, 2}, {3, 4}}) == 2);
int main() { return 0; }
