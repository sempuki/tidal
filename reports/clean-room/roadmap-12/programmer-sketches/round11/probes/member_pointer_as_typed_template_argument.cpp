struct Counter { int n = 0; int add(int v) { return n += v; } };
template <int (Counter::*Mem)(int)> int call(Counter& obj) { return (obj.*Mem)(2); }
int main() {
  Counter c;
  return call<&Counter::add>(c) == 2 ? 0 : 1;
}
