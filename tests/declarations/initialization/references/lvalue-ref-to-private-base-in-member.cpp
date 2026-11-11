// tidal-rules: REF-1 ACC-12
// tidal-expect: run
// tidal-note: G153. Inside a member of Sealed, the private base Core is accessible (ACC-12). The Core& binds directly to the Core subobject of the Sealed lvalue, so the reference and the subobject are the same object.
struct Core { int v = 1; };
struct Sealed : private Core {
  bool same_object() {
    Core& c = *this;
    c.v = 5;
    return &c == static_cast<Core*>(this) && v == 5;
  }
};
int main() {
  Sealed s;
  return s.same_object() ? 0 : 1;
}
