// tidal-rules: CLS-5
// tidal-expect: run
struct Cache {
  mutable int hits = 0;
  int get() const { ++hits; return 1; }
};
int main() {
  const Cache c;
  c.get(); c.get();
  return c.hits == 2 ? 0 : 1;
}
