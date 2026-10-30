// tidal-rules: INIT-2
// tidal-expect: accept
constexpr bool check() {
  int a{};
  int b = int();
  int c = {};
  double d{};
  int* p{};
  bool e{};
  int arr[3]{};
  return a == 0 && b == 0 && c == 0 && d == 0.0 && p == nullptr && !e && arr[0] == 0 && arr[2] == 0;
}
static_assert(check());
