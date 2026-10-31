// tidal-rules: UNI-1
// tidal-expect: run
union U {
  int i;
  float f;
  U() : i(3) {}
  int get() const { return i; }
};
int main() { U u; return u.get() == 3 ? 0 : 1; }
