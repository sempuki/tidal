// tidal-rules: NAM-21
// tidal-expect: run
// tidal-note: Gap review of the text phase 8 wrote on expressions, foundations, and templates. The base clause of a class defined with a qualified name sees the members of the namespace without qualification. Clang 22, GCC 16, and EDG 7.0 accept.
namespace lib {
  struct Root { int r = 1; };
  struct Leaf;
}
struct lib::Leaf : Root { int x = 4; };
int main() { lib::Leaf leaf; return leaf.r + leaf.x == 5 ? 0 : 1; }
