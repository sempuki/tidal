consteval int twice(int v) { return 2 * v; }
template <class T> constexpr T wrap(T v) { return twice(v); }
static_assert(wrap(2) == 4);
int main(int argc, char**) { return wrap(argc) - 2 * argc; }
