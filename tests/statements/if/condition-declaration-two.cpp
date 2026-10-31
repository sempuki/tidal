// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-edg: condition declaration can only declare one variable
void f() {
  if (int a = 1, b = 2) {}
}
