struct Big { int a[100]; };
int main() {
  Big b{};
  auto f = [b] { return sizeof(decltype(b)); };
  static_assert(sizeof(f) >= sizeof(Big));   // the explicit capture makes a member
  return f() == sizeof(Big) ? 0 : 1;
}
