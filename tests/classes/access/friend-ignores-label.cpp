// tidal-rules: ACC-5
// tidal-expect: run
class C {
  int secret = 3;
public:
  friend int reveal(const C&);
};
class D {
  int secret = 4;
private:
  friend int reveal(const D&);
};
int reveal(const C& c) { return c.secret; }
int reveal(const D& d) { return d.secret; }
int main() { return reveal(C{}) + reveal(D{}) == 7 ? 0 : 1; }
