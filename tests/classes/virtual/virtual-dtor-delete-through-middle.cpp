// tidal-rules: VIRT-7 VIRT-12
// tidal-expect: run
// tidal-note: Only the root class declares a virtual destructor. The implicit destructor of the middle class overrides it, so a delete through a pointer to the middle class is not undefined and runs the destructor of the most derived class.
int destroyed = 0;
struct Root { virtual ~Root() = default; };
struct Middle : Root {};
struct Leaf : Middle { ~Leaf() { ++destroyed; } };
int main() {
  Middle* p = new Leaf;
  delete p;
  return destroyed == 1 ? 0 : 1;
}
