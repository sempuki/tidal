// tidal-rules: CPY-27 UNI-11
// tidal-expect: reject
// tidal-error-clang: implicitly.deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: The member of Item is an array of Part, and the copy of each element selects the user-provided copy constructor. So the copy constructor of Item is not trivial, and the union deletes its own.
struct Part { Part() = default; Part(const Part&) {} };
struct Item { Part parts[2]; };
union Slot {
  Item item;
  int raw;
  Slot() : raw(0) {}
};
void run() {
  Slot a;
  Slot b = a;
}
