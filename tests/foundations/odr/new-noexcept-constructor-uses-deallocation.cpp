// tidal-rules: ODR-5 ODR-8
// tidal-expect: link-fail
// tidal-expect-clang: run
// tidal-expect-gcc-14: run
// tidal-expect-gcc-16-o2: run
// tidal-error-gcc: undefined reference to .Node::operator
// tidal-sources: new-uses-deallocation-function.tu.cpp
// tidal-note: Q-0358. The new expression uses Node::operator delete, which has no definition, also when the constructor is noexcept. GCC 16 at -O0 refers to the function and fails to link. Clang, GCC 14, and GCC 16 at -O2 emit no reference and link (ODR-8 needs no diagnostic).
int start();
struct Node {
  int v;
  Node() noexcept : v(start()) {}
  static void* operator new(decltype(sizeof 0));
  static void operator delete(void*);
};
alignas(Node) static char pool[sizeof(Node)];
void* Node::operator new(decltype(sizeof 0)) { return pool; }
int main() { Node* node = new Node; return node->v - 1; }
