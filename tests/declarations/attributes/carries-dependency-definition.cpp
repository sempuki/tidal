// tidal-rules: ATTR-11
// tidal-expect: run
// tidal-note: A definition with the attribute on the function and the parameter. A call in a program with one thread works as without the attribute.
[[carries_dependency]] int* pass([[carries_dependency]] int* p) { return p; }
int main() {
  int v = 4;
  return *pass(&v) == 4 ? 0 : 1;
}
