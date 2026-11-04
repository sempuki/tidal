// tidal-rules: ODR-5 ODR-8
// tidal-expect: run
// tidal-expect-gcc: link-fail
// tidal-expect-gcc-14: run
// tidal-expect-gcc-16-o2: run
// tidal-error-gcc: undefined reference to .Node::operator
// tidal-sources: new-uses-deallocation-function.tu.cpp
// tidal-note: Q-0358. The constructor is noexcept, so it cannot throw, and the new expression does not use Node::operator delete, which has no definition. GCC 16 refers to the deallocation function at -O0 also for a noexcept constructor. GCC 14, and GCC 16 at -O2, do not.
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
