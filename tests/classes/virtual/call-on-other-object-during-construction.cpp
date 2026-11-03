// tidal-rules: VIRT-6 VIRT-1
// tidal-expect: run
int seen = 0;
struct Base {
  virtual int id() const { return 1; }
  Base() {}
  Base(const Base& source) { seen = source.id(); }  // source is complete
};
struct Derived : Base { int id() const override { return 2; } };
int main() {
  Derived d;
  Base copy(d);          // the call through source uses Derived::id
  return seen == 2 ? 0 : 1;
}
