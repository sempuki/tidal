// tidal-rules: THR-4
// tidal-expect: run
// tidal-note: The operand is a dereferenced pointer to the base, and the handlers take the objects by value.
struct B { int tag = 1; virtual ~B() {} };
struct D : B { D() { tag = 2; } };
int main() {
  D d;
  B* p = &d;
  try {
    throw *p;
  } catch (D) {
    return 1;
  } catch (B e) {
    return e.tag == 2 ? 0 : 2;
  }
  return 3;
}
