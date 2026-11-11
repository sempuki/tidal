// A pointer to member function as the explicit argument of an auto
// template parameter. From signal.cpp.
struct Counter { int n = 0; int add(int v) { return n += v; } };
template <auto Mem> int call(Counter& obj) { return (obj.*Mem)(2); }
int main() {
  Counter c;
  return call<&Counter::add>(c) == 2 ? 0 : 1;
}
