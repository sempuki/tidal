// tidal-rules: VIRT-20
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: accept
// tidal-expect-clang-22-o2: accept
// tidal-error-clang: undefined reference to .vtable
// tidal-error-gcc: undefined reference to .vtable
// tidal-note: EDG does not link. At -O2, GCC and Clang emit no reference to the vtable, and the program links (VIRT-20 needs no diagnostic).
struct S { virtual void f(); };
int main() { S s; return 0; }
