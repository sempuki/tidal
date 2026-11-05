// tidal-rules: VIRT-7 VIRT-12
// tidal-expect: run
// tidal-note: The middle class declares a destructor without the virtual keyword. The declared destructor overrides the virtual destructor of the root, so a delete through a pointer to the middle class runs the destructor of the most derived class, then the middle one.
int order = 0;
struct Root { virtual ~Root() = default; };
struct Middle : Root { ~Middle() { order = order * 10 + 2; } };
struct Leaf : Middle { ~Leaf() { order = order * 10 + 1; } };
int main() {
  Middle* p = new Leaf;
  delete p;
  return order == 12 ? 0 : 1;
}
