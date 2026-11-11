template <class M> struct traits;
template <class R, class C> struct traits<R (C::*)(int)> { using cls = C; };
struct Counter { int n = 0; int add(int v) { return n += v; } };
template <auto Mem> int call(typename traits<decltype(Mem)>::cls& obj) { return (obj.*Mem)(2); }
int main() {
  Counter c;
  return call<&Counter::add>(c) == 2 ? 0 : 1;
}
