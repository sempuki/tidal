// tidal-rules: NEW-13
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int class_calls = 0;
alignas(16) unsigned char buf[64];
struct S { int v;
  static void* operator new(decltype(sizeof 0)) { ++class_calls; return buf; }
  static void operator delete(void*) {} };
int main() { S* a = new S; S* b = ::new S; bool ok = class_calls == 1 && a != b; ::delete b; return ok ? 0 : 1; }
