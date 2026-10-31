// tidal-rules: LAM-9
// tidal-expect: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-error-gcc: .a. is not captured
// tidal-error-edg: enclosing lambda does not allow implicit captures
int f() {
  int a = 1;
  auto outer = [] {
    auto inner = [a] { return a; };
    return inner();
  };
  return outer();
}
