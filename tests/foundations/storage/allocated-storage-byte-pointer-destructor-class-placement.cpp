// tidal-rules: STG-28
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Roadmap round 12, programmer probe placement_destructor_class_through_byte_pointer.cpp. Cell has a user destructor, so Cell is not an implicit-lifetime type. A Cell* made through void* from the unsigned char* result of the allocation function points to element 0 of an array of Cell (STG-28). Placement new builds the element, the program reads it, and the destructor call ends its lifetime before the storage is freed.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
struct Cell {
  int tag;
  ~Cell() {}
};
int main() {
  unsigned char* raw = static_cast<unsigned char*>(::operator new(4 * sizeof(Cell)));
  Cell* first = static_cast<Cell*>(static_cast<void*>(raw));
  ::new (first) Cell{8};
  int result = first->tag == 8 ? 0 : 1;
  first->~Cell();
  ::operator delete(raw);
  return result;
}
