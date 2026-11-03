constexpr bool loop(unsigned n) { unsigned s = 0; for (unsigned i = 0; i < n; ++i) s += i; return s != 1; }
static_assert(loop(300000));
int main() { return 0; }
