// tidal-rules: CONV-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct S { int v; };
int main() {
  int S::* null_pm = nullptr;
  int S::* pm = &S::v;
  if (null_pm) return 1;
  if (!pm) return 2;
  return 0;
}
