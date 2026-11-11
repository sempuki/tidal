// The same with a union member of a class element (the hash map slot form).
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct KV { int key; int value; };
struct Slot {
  union { KV kv; };
  Slot() {}
};
int main() {
  Slot* slots = new Slot[2];
  ::new (static_cast<void*>(&slots[0].kv)) KV{1, 2};
  int r = slots[0].kv.value == 2 ? 0 : 1;
  delete[] slots;
  return r;
}
