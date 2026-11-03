consteval int twice(int v) { return 2 * v; }
constexpr int wrap(int v) { return twice(v); }
static_assert(wrap(2) == 4);
int main() { return 0; }
