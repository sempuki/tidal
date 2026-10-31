// tidal-rules: LAM-8
// tidal-expect: reject
// tidal-error-clang: .this. cannot be captured in this context
// tidal-error-gcc: invalid use of .this. in non-member function
// tidal-error-edg: may only be used inside a nonstatic member function
void f() {
  auto g = [this] { return 1; };
}
