// tidal-rules: ODR-5 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Node::operator
// tidal-error-gcc: undefined reference to .Node::operator
// tidal-sources: new-uses-deallocation-function.tu.cpp
int start();
struct Node {
  int v;
  Node() : v(start()) {}
  static void* operator new(decltype(sizeof 0));
  static void operator delete(void*);
};
alignas(Node) static char pool[sizeof(Node)];
void* Node::operator new(decltype(sizeof 0)) { return pool; }
int main() { Node* node = new Node; return node->v - 1; }
