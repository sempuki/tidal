// tidal-rules: DEP-9 LAM-15
// tidal-expect: run
// tidal-note: The generic lambda is at namespace scope, and its call operator is a template. The call probe(item) is dependent, so argument-dependent lookup at the point of instantiation finds lib::probe, which is declared after the lambda.
namespace lib { struct Item {}; }
auto info = [](auto item) { return probe(item); };
namespace lib { constexpr int probe(Item) { return 4; } }
int use() { return info(lib::Item{}); }
static_assert(info(lib::Item{}) == 4);
int main() { return use() == 4 ? 0 : 1; }
