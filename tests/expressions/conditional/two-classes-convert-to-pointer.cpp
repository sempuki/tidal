// tidal-rules: COND-11
// tidal-expect: run
// tidal-note: Two unrelated classes that both convert to int* give an int* prvalue.
int first = 1, second = 2;
struct A { operator int*() const { return &first; } };
struct B { operator int*() const { return &second; } };
int main() {
  A a;
  B b;
  int* p = true ? a : b;
  int* q = false ? a : b;
  return p == &first && q == &second ? 0 : 1;
}
