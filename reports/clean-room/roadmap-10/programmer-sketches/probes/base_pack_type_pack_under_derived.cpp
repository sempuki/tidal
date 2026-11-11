// A pack of base types (Bs(v)...) inside a class that a derived class builds.
struct A { int value; A(int v) : value(v) {} };
struct B { int value; B(int v) : value(v + 1) {} };
template <class... Bs> struct Impl : Bs... {
  Impl(int v) : Bs(v)... {}
};
struct Top : Impl<A, B> {
  Top(int v) : Impl<A, B>(v) {}
};
int main() {
  Top t(5);
  A& a = t;
  B& b = t;
  return (a.value == 5 && b.value == 6) ? 0 : 1;
}
