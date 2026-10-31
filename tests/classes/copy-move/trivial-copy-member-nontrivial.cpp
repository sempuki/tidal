// tidal-rules: CPY-27 UNI-11
// tidal-expect: reject
// tidal-error-clang: implicitly.deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
struct Part { Part() = default; Part(const Part&) {} };
struct Item { Part p; };
union Slot {
  Item item;
  int raw;
  Slot() : raw(0) {}
};
void run() {
  Slot a;
  Slot b = a;
}
