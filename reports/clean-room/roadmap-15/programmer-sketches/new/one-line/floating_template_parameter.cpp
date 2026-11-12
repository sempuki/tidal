template <double Scale> constexpr double scaled(double x) { return x * Scale; }
static_assert(scaled<2.5>(4.0) == 10.0);
int main() { return scaled<0.5>(6.0) == 3.0 ? 0 : 1; }
