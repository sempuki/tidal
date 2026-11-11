struct Counter { int n = 0; int add(int v) { return n += v; } };
template <auto Mem> int call(Counter& obj) { return (obj.*Mem)(2); }
int main() {
  Counter c;
  constexpr auto m = &Counter::add;
  return call<m>(c) == 2 ? 0 : 1;
}
