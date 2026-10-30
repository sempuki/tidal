// tidal-rules: ACC-1
// tidal-expect: accept
class C {
public:
  int a;
private:
  int b;
public:
  int get() const { return b; }
};
int f(C c) { return c.a + c.get(); }
