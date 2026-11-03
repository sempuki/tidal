// tidal-rules: TRY-2
// tidal-expect: run
struct B { int m; };
struct D : B { int n; };
int main() {
  try { throw &B::m; }                      // int B::*
  catch (int D::*) { return 1; }            // no match: no conversion applies
  catch (int B::*) { return 0; }
  return 2;
}
