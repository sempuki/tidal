consteval int twice(int v) { return 2 * v; }
constexpr int f(int v) {
  if consteval { return twice(v); } else { return v + v; }
}
static_assert(f(3) == 6);
int main(int argc, char**) { return f(argc) == 2 * argc ? 0 : 1; }
