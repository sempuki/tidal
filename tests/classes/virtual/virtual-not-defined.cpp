// tidal-rules: VIRT-20
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .vtable
// tidal-error-gcc: undefined reference to .vtable
// tidal-note: Gap review 2026-10-31. EDG does not link.
struct S { virtual void f(); };
int main() { S s; return 0; }
