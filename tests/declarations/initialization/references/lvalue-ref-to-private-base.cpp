// tidal-rules: REF-1 ACC-12
// tidal-expect: reject
// tidal-error-clang: cannot cast .* to its private base class
// tidal-error-gcc: inaccessible base of
// tidal-error-edg: inaccessible
// tidal-note: I5-10, I5 reduction r2-reference-to-private-base.cpp. Sealed derives from Core through a private base. Outside Sealed, the conversion to Core needs access to the base (ACC-12), so the Core& parameter cannot bind to a Sealed lvalue (REF-1).
struct Core { int v = 1; };
struct Sealed : private Core {};
int touch(Core& c) { return c.v; }
int main() {
  Sealed s;
  return touch(s);
}
