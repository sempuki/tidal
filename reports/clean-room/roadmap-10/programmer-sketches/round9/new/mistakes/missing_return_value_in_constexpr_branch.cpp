constexpr int sign(int x) { if (x > 0) return 1; if (x < 0) return -1; }
static_assert(sign(0) == 0);
int main() {}
