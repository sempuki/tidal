// tidal-rules: VIRT-12
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: pure virtual method called
// tidal-note: Run-time check (decision 0028). The run-time library of GCC and Clang stops the virtual call of a pure virtual function, with or without the sanitizer.
struct Shape {
  Shape() { describe(); }
  int describe() { return area(); }   // a virtual call
  virtual int area() = 0;
};
struct Square : Shape { int area() override { return 4; } };
int main() {
  Square square;   // undefined: Shape() reaches the pure area
  return 0;
}
