// tidal-rules: ODR-5 ODR-8
// tidal-expect: run
// tidal-sources: new-uses-deallocation-function.tu.cpp
// tidal-note: The definition of the constructor uses Node::operator new and Node::operator delete, which have no definitions, so the program is not valid. No new expression names them, Clang and GCC emit no reference, and the program links (ODR-8 needs no diagnostic).
int start();
struct Node {
  int v;
  Node() : v(start()) {}
  static void* operator new(decltype(sizeof 0));
  static void operator delete(void*);
};
int main() { Node node; return node.v - 1; }
