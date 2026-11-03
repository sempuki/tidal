// tidal-rules: CLS-19 CLS-30
// tidal-expect: run
// tidal-note: Every data member of Leaf is declared in Root, two bases down, and the classes between declare none. So Leaf is standard-layout, and the object, its bases, and its first member share one address.
template <class A, class B> bool same_address(const A& a, const B& b) { return static_cast<const void*>(&a) == static_cast<const void*>(&b); }
struct Root { int first; int second; };
struct Middle : Root {};
struct Leaf : Middle { int get() const { return first; } };
int main() {
  Leaf leaf{};
  return same_address(leaf, leaf.first) && same_address(leaf, static_cast<const Root&>(leaf)) && same_address(leaf, static_cast<const Middle&>(leaf)) ? 0 : 1;
}
