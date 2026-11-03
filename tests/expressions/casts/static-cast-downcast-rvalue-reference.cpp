// tidal-rules: CAST-15
// tidal-expect: run
// tidal-note: The cast gives an rvalue reference to the derived class from an lvalue of the base.
struct B { int b = 1; };
struct D : B { int d = 2; };
int main() {
  D obj;
  B& r = obj;
  D&& moved = static_cast<D&&>(r);
  return &moved == &obj && moved.d == 2 ? 0 : 1;
}
