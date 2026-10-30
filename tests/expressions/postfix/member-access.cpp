// tidal-rules: PFX-10
// tidal-expect: run
struct S { int v; static int s; int get() const { return v; } };
int S::s = 4;
int main() {
  S obj{3};
  S* ptr = &obj;
  return obj.v == 3 && ptr->v == 3 && (*ptr).get() == 3 && obj.s == 4 ? 0 : 1;
}
