// A comma with a name as its left operand in a template.
// Default: the template is never instantiated. -DUSE: instantiated with int.
// -DDEFER_CLASS: the same, with a class whose operator+ is a function.
// -DDEFER: the left operand is an operator expression with dependent operands.
template <class T> T pick(T a, T b) { return (a, b); }
template <class T> T sum(T a, T b) { return (a + b, b); }
struct C { int v; };
C operator+(C l, C r) { return {l.v + r.v}; }
int main() {
#ifdef USE
  return pick(1, 0);
#elif defined(DEFER_CLASS)
  return sum(C{1}, C{0}).v;
#elif defined(DEFER)
  return sum(C{1}, C{0}).v + sum(1, 0);
#else
  return 0;
#endif
}
