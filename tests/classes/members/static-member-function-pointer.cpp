// tidal-rules: CLS-7
// tidal-expect: run
// tidal-note: The address of a static member function is an ordinary pointer to function. A call through a pointer to an object also works.
struct S {
  static int add(int a, int b) { return a + b; }
};
int main() {
  int (*p)(int, int) = &S::add;
  S s;
  S* ps = &s;
  return p(1, 2) == 3 && ps->add(2, 2) == 4 ? 0 : 1;
}
