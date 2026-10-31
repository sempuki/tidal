// tidal-rules: MPTR-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int f() { return 3; } };
struct T { long g(int) { return 0; } };
int main() { using PF = int (S::*)(); using PT = long (T::*)(int);
  PT t = reinterpret_cast<PT>(&S::f); PF back = reinterpret_cast<PF>(t);
  S s; return back == &S::f && (s.*back)() == 3 ? 0 : 1; }
