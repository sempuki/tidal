// tidal-rules: ACC-10
// tidal-expect: run
class Outer {
  int secret = 8;
public:
  struct Inner { int look(const Outer& o) const { return o.secret; } };
};
int main() { return Outer::Inner{}.look(Outer{}) == 8 ? 0 : 1; }
