// tidal-rules: ACC-5
// tidal-expect: run
class C;
struct Reader { int read(const C& c) const; int other(const C& c) const; };
class C {
  int secret = 9;
  friend int Reader::read(const C&) const;
};
int Reader::read(const C& c) const { return c.secret; }
int main() { return Reader{}.read(C{}) == 9 ? 0 : 1; }
