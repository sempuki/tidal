struct Counter { int n = 0; int add(int v) { return n += v; } };
template <auto Mem> struct Bound { static int call(Counter& obj) { return (obj.*Mem)(2); } };
int main() {
  Counter c;
  return Bound<&Counter::add>::call(c) == 2 ? 0 : 1;
}
