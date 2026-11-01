// tidal-rules: TYN-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: changes meaning of
// tidal-note: Q-0066. The member declaration of cells uses the global function f, and the later data member f gives the name another meaning in the class. TYN-5 needs no diagnostic. GCC rejects the class, and Clang accepts it.
int f(int) { return 1; }
struct Table {
  int cells[sizeof(f(0))];
  int f;
};
