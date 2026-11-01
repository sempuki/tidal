// tidal-rules: FN-15 FN-2 DCL-8
// tidal-expect: run
// tidal-note: After the adjustments of DCL-8, each pair of declarations has the same parameter types, so each pair declares one function. Each function has one definition.
int scale(int);
int scale(const int);
int scale(int v) { return v * 2; }
int first(int*);
int first(int[4]);
int first(int values[]) { return values[0]; }
int apply(int(int));
int apply(int (*f)(int)) { return f(3); }
int main() {
  int data[] = {5};
  return scale(2) == 4 && first(data) == 5 && apply(scale) == 6 ? 0 : 1;
}
