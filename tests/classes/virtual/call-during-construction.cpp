// tidal-rules: VIRT-6
// tidal-expect: run
int seen = 0;
struct B {
  B() { seen = f(); }
  virtual int f() { return 1; }
};
struct D : B { int f() override { return 2; } };
int main() { D d; B& r = d; return seen == 1 && r.f() == 2 ? 0 : 1; }
