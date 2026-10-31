// tidal-rules: NEW-20
// tidal-expect: reject
// tidal-error-clang: no matching .*operator new
// tidal-error-gcc: no matching function for call to .operator new
// tidal-error-edg: no instance of overloaded .*operator new
int main() {
  int x;
  int* p = new (&x) int(3);
  return *p == 3 ? 0 : 1;
}
