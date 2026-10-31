// tidal-rules: ODR-6 ODR-8
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error-clang: undefined reference to .vtable
// tidal-error-gcc: undefined reference to .vtable
// tidal-note: The missing definition of draw leaves Widget without a virtual table. At -O2, GCC and Clang call size directly and emit no reference to the virtual table, so the program links (ODR-8 needs no diagnostic).
struct Widget {
  virtual int draw();
  virtual int size() { return 1; }
};
int main() { Widget widget; return widget.size() - 1; }
