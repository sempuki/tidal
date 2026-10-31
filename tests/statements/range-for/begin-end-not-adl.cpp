// tidal-rules: RFOR-7
// tidal-expect: reject
// tidal-error-clang: no viable .begin. function
// tidal-error-gcc: begin. was not declared in this scope
// tidal-error-edg: requires a suitable .begin. function
// tidal-note: Gap review 2026-10-30.
namespace data { struct Bag { int d[2]; }; }
namespace helpers {
  int* begin(data::Bag& b) { return b.d; }
  int* end(data::Bag& b) { return b.d + 2; }
}
using namespace helpers;
void f(data::Bag& bag) { for (int v : bag) (void)v; }
