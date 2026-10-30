// tidal-rules: ACC-5
// tidal-expect: run
class C {
  int secret = 6;
  friend class Inspector;
};
class Inspector {
public:
  int look(const C& c) const { return c.secret; }
};
int main() { return Inspector{}.look(C{}) == 6 ? 0 : 1; }
