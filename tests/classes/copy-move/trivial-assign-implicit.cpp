// tidal-rules: CPY-27 UNI-11
// tidal-expect: accept
struct Item { int v; };
union Slot {
  Item item;
  int raw;
  Slot() : raw(0) {}
};
void run() {
  Slot a;
  Slot b; b = a;
}
