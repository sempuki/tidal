// tidal-rules: CPY-27 UNI-11
// tidal-expect: accept
struct Item { int v = 0; };
union Slot {
  Item item;
  int raw;
  Slot() : raw(0) {}
};
void run() {
  Slot a;
  Slot b = a;
}
