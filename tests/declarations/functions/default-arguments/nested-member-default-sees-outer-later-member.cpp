// tidal-rules: DEFARG-10 DEFARG-3
// tidal-expect: run
// tidal-note: A member function of a nested class has its default argument checked as if at the end of the outermost enclosing class, so the default can name a static member of the outer class declared after the nested class.
struct Outer {
  struct Inner { int get(int n = depth) { return n; } };
  static const int depth = 7;
};
int main() { return Outer::Inner{}.get() == 7 ? 0 : 1; }
