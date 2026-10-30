// tidal-rules: ACC-5
// tidal-expect: run
class C {
  int secret = 4;
  friend int reveal(const C&);
};
int reveal(const C& c) { return c.secret; }
int main() { return reveal(C{}) == 4 ? 0 : 1; }
