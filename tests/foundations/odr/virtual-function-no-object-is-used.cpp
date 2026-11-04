// tidal-rules: ODR-6 ODR-8
// tidal-expect: run
// tidal-note: draw is virtual and not pure, so it is used, and the program is not valid without its definition. The program creates no Widget, Clang and GCC emit no reference to the virtual table, and the program links (ODR-8 needs no diagnostic).
struct Widget {
  virtual int draw();
  virtual int size() { return 1; }
};
int main() { return 0; }
