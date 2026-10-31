// tidal-rules: VIRT-20
// tidal-expect: link-fail
// tidal-note: Gap review 2026-10-31. EDG does not link.
struct S { virtual void f(); };
int main() { S s; return 0; }
