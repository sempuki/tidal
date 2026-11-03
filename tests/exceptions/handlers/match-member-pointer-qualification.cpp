// tidal-rules: TRY-2
// tidal-expect: run
struct B { int m; };
struct D : B { int n; };
int qualification() {
  try { throw &D::n; }                      // int D::*
  catch (const int D::* p) { return p == &D::n ? 0 : 1; }
  catch (...) { return 2; }
  return 3;
}
int inherited() {
  try { throw &D::m; }                      // &D::m has the type int B::*
  catch (int B::* p) { return p == &B::m ? 0 : 4; }
  catch (...) { return 5; }
  return 6;
}
int main() { return qualification() + inherited(); }
