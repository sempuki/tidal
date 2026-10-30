// tidal-rules: RFOR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
namespace data { struct Bag { int d[2]; }; }
namespace helpers {
  int* begin(data::Bag& b) { return b.d; }
  int* end(data::Bag& b) { return b.d + 2; }
}
using namespace helpers;
void f(data::Bag& bag) { for (int v : bag) (void)v; }
