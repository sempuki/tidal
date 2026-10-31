// tidal-rules: LAM-26
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .decltype\(g\).
// tidal-error-gcc: use of deleted function .f\(\)::<lambda\(\)>::<lambda>\(\).
// tidal-error-edg: default constructor of .lambda.* it is a deleted function
void f() {
  int n = 0;
  auto g = [n] { return n; };
  decltype(g) h;
}
