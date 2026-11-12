template <class... T> constexpr int sum_later(T... xs) {
  auto f = [... ys = xs] { return (ys + ... + 0); };
  return f();
}
static_assert(sum_later(1, 2, 3) == 6);
int main() { return sum_later() ; }
