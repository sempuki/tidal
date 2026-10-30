// tidal-rules: ASN-6
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct V {
  int x, y;
  V(int a, int b) : x(a), y(b) {}
  V& operator+=(V o) { x += o.x; y += o.y; return *this; }
};
void f(V& v) { v += {1, 2}; }
