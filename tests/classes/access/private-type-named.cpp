// tidal-rules: ACC-3
// tidal-expect: reject
class Outer {
  struct Hidden { int v; };
public:
  static Hidden make();
};
Outer::Hidden h;
