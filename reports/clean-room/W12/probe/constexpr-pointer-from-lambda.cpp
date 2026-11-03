constexpr int (*p)(int) = [](int x) { return x + 1; };
static_assert(p(1) == 2);
int main() { return p(-1); }
