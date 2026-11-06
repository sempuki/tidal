// A lambda whose trailing return type is decltype(auto): a plain lambda,
// and a generic lambda that is also mutable.
int main() {
  auto p = [](int x) -> decltype(auto) { return x; };
  auto q = [](auto x) mutable -> decltype(auto) { return x; };
  return p(0) + q(0);
}
