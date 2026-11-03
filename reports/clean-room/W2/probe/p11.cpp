constexpr double nan = 0.0 / 0.0;
static_assert(nan != nan);
int main() { return 0; }
