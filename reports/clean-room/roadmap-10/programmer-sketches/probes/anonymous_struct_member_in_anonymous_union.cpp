// An unnamed struct type as the type of a member of an anonymous union
// in a class (the hash map's first try). UNI-9 forbids nested types.
struct Slot {
  union { struct { int key; int value; } kv; };
  Slot() {}
};
int main() { Slot s; s.kv.key = 1; return s.kv.key == 1 ? 0 : 1; }
