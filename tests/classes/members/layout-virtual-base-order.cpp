// tidal-rules: CLS-35 CLS-34
// tidal-expect: run
// tidal-note: x86-64. The virtual bases come in the order that a depth-first walk of the base lists, from left to right, meets them.
template <class Whole, class Part> int offset(Whole& w) {
  return int(reinterpret_cast<char*>(static_cast<Part*>(&w)) - reinterpret_cast<char*>(&w));
}
struct First { int a; };
struct Other { int b; };
struct Left : virtual Other { char l; };
struct Right : virtual First { char r; };
// Left meets Other before Right meets First
struct Graph : Left, Right { char g; };
// Right meets First before the direct virtual base Other
struct Deep : Right, virtual Other {};
// the direct virtual base Other comes first in the base list
struct Direct : virtual Other, Right {};
int main() {
  Graph g;
  Deep d;
  Direct e;
  if (offset<Graph, Right>(g) != 16 || offset<Graph, Other>(g) != 28 || offset<Graph, First>(g) != 32) return 1;
  if (offset<Deep, First>(d) != 12 || offset<Deep, Other>(d) != 16) return 2;
  if (offset<Direct, Other>(e) != 12 || offset<Direct, First>(e) != 16) return 3;
  return 0;
}
