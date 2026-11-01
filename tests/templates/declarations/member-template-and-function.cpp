// tidal-rules: TPL-44
// tidal-expect: run
// tidal-note: A plain member function and a member function template that can give the same signature. The call without a list picks the plain function, and the empty list picks the template.
struct Tool {
  int use(int) { return 1; }
  template <class T> int use(T) { return 2; }
};
int main() { Tool t; return t.use(0) == 1 && t.use<>(0) == 2 ? 0 : 1; }
