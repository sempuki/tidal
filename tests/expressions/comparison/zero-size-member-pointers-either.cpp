// tidal-rules: CMP-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31. An empty member that shares its address with the next member has no order with it.
struct Empty {};
struct S {
  [[no_unique_address]] Empty tag;
  int value;
};
int main() {
  S s{};
  const void* a = &s.tag;
  const void* b = &s.value;
  bool lt = a < b;
  bool gt = a > b;
  return (lt || gt || !lt) ? 0 : 1;
}
