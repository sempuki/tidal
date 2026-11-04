// tidal-rules: PFX-11
// tidal-expect: accept
// tidal-note: A member function body sees every member of its class, also a member declared after the function.
struct Gauge {
  int read() { return this->later; }
  int later = 3;
};
int use() {
  Gauge g;
  return g.read();
}
