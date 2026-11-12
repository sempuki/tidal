// tidal-rules: ATTR-19
// tidal-expect: run
// tidal-note: RV10-3. An attribute specifier comes before a using-directive, at namespace scope and in a block. Every compiler accepts both.
namespace tools { int count = 2; }
[[]] using namespace tools;
int main() {
  [[]] using namespace tools;
  return count == 2 ? 0 : 1;
}
