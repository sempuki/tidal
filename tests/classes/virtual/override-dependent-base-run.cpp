// tidal-rules: VIRT-3 DEP-28
// tidal-expect: run
// tidal-note: The base is a template parameter. Wrapper<HasRun> overrides HasRun::run, and a call through the base reaches the override.
struct HasRun {
  virtual int run() { return 1; }
  virtual ~HasRun() = default;
};
template <class Base> struct Wrapper : Base { int run() override { return 2; } };
int main() {
  Wrapper<HasRun> w;
  HasRun& b = w;
  return b.run() == 2 ? 0 : 1;
}
