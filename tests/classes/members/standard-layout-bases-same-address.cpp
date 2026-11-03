// tidal-rules: CLS-19
// tidal-expect: run
struct Tag {};
struct Mark {};
struct Item : Tag, Mark { int v; };
int main() {
  Item item;
  void* whole = &item;
  void* tag = static_cast<Tag*>(&item);
  void* mark = static_cast<Mark*>(&item);
  void* first = &item.v;
  return whole == tag && whole == mark && whole == first ? 0 : 1;
}
