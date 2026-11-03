// tidal-rules: LAM-24
// tidal-expect: reject
// tidal-error-clang: .this. cannot be captured in this context
// tidal-error-gcc: .this. is unavailable for static member functions
// tidal-error-edg: may only be used inside a nonstatic member function
struct S {
  int v = 0;
  static int f() {
    auto g = [this] { return 1; };
    return g();
  }
};
