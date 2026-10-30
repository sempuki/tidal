// tidal-rules: ACC-11
// tidal-expect: accept
class Outer {
  struct Hidden { int v = 3; };
public:
  static Hidden make() { return {}; }
};
auto h = Outer::make();
int x = h.v;
