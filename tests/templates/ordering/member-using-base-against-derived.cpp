// tidal-rules: ORD-6
// tidal-expect: run
// tidal-note: G147. A using declaration brings a base member template into the derived class. Both object parameters name the derived class, so f(T*) is more specialized and the call chooses it.
struct A { template <class T> int f(T*) { return 1; } };
struct B : A {
  using A::f;
  template <class T> int f(T) { return 2; }
};
int main() {
  B b;
  int i = 0;
  return b.f(&i) == 1 ? 0 : 1;
}
