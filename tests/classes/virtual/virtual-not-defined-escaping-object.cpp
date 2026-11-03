// tidal-rules: VIRT-20
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .vtable
// tidal-error-gcc: undefined reference to .vtable
// tidal-note: The object escapes through a volatile pointer, so its constructor must store a pointer to the vtable also at -O2. No code calls f.
struct S { virtual void f(); int v = 0; };
S* volatile keep;
int main() {
  keep = new S;
  return 0;
}
