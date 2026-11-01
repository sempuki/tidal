// tidal-rules: DEP-11
// tidal-expect: run
int count = 5;
struct Base { int count = 1; };
template <class T> struct Derived : Base {
  int get() { return count; }
};
int main() { return Derived<int>{}.get() == 1 ? 0 : 1; }
