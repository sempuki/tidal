// tidal-rules: ACC-11
// tidal-expect: accept
class Outer {
  struct Hidden { int v; };
  static Hidden make();
};
Outer::Hidden Outer::make() { return Hidden{1}; }
