// A lambda with a template head (LAM-15), called with deduction.
// g++ and clang++ accept, and the program exits 0.
// tidal-fe: unsupported at line 0: object of unknown type.
int main() {
  auto id = []<class T>(T x) { return x; };
  return id(0);
}
