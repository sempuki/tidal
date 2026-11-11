// tidal-rules: STG-28
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Gap review RV8-A-1. Slot has a user-provided copy constructor and no other trivial constructor, so Slot is not an implicit-lifetime type (STG-7). The allocation function creates an array of Slot (STG-28). A Slot* cast through void* from the byte at the start of element 1 points to element 1. Placement new there builds the element, and the read goes through the cast pointer, not through the pointer that placement new returns. The review reads the cast and the read through it as undefined (Q-0430). Every compiler that runs the program reads the value, and no sanitizer reports it.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
struct Slot {
  int tag;
  Slot(int t) : tag(t + 1) {}
  Slot(const Slot& other) : tag(other.tag) {}
};
int main() {
  unsigned char* raw = static_cast<unsigned char*>(::operator new(4 * sizeof(Slot)));
  Slot* second = static_cast<Slot*>(static_cast<void*>(raw + sizeof(Slot)));
  ::new (static_cast<void*>(second)) Slot(6);
  int result = second->tag == 7 ? 0 : 1;
  ::operator delete(raw);
  return result;
}
