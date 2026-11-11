// tidal-rules: STG-28
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Roadmap round 12, programmer reduction placement_class_through_byte_pointer_into_allocated_storage.cpp (the cause of the stop of round 11's arena_fourth_try.cpp). The program keeps the result of the allocation function as unsigned char* and makes a Cell* to its start through void*. Cell has a user-provided copy constructor and no other trivial constructor, so Cell is not an implicit-lifetime type (STG-7, gap review RV8-A-2), and STG-28 still gives an array of Cell whose element 0 is at that address. Placement new builds element 0, and the read through the Cell* reaches it.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
struct Cell {
  int tag;
  Cell(int t) : tag(t * 3) {}
  Cell(const Cell& other) : tag(other.tag) {}
};
int main() {
  unsigned char* raw = static_cast<unsigned char*>(::operator new(4 * sizeof(Cell)));
  Cell* first = static_cast<Cell*>(static_cast<void*>(raw));
  ::new (first) Cell(5);
  int result = first->tag == 15 ? 0 : 1;
  ::operator delete(raw);
  return result;
}
