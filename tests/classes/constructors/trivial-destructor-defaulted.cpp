// tidal-rules: DTOR-9 UNI-11
// tidal-expect: accept
struct Item { int v; ~Item() = default; };
union Slot {
  Item item;
  int raw;
  Slot() : raw(0) {}
};
void run() { Slot s; }
