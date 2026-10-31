// tidal-rules: VIRT-6
// tidal-expect: run
int seen = 0;
struct B {
  virtual ~B() { seen = f(); }
  virtual int f() { return 1; }
};
struct D : B { int f() override { return 2; } };
int main() { { D d; } return seen == 1 ? 0 : 1; }
