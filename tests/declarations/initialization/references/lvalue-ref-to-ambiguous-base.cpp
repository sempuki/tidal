// tidal-rules: REF-1 CONV-17
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: ambiguous base of
// tidal-error-edg: ambiguous
// tidal-note: I5-10, I5 reduction r3-reference-to-ambiguous-base.cpp. Joined has two Core subobjects, one through Left and one through Right. The conversion to Core is ambiguous (CONV-17), so the Core& parameter cannot bind to a Joined lvalue (REF-1).
struct Core { int v = 1; };
struct Left : Core {};
struct Right : Core {};
struct Joined : Left, Right {};
int touch(Core& c) { return c.v; }
int main() {
  Joined j;
  return touch(j);
}
