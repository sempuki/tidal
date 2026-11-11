// tidal-rules: STG-28 ADD-14
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Roadmap round 11, programmer reduction placement_at_typed_step_after_byte_offset_in_allocated_storage.cpp. The allocation function creates an array of long in its storage (STG-28). A long* made from the address 2 * sizeof(long) bytes in points to element 2 of a long array there, so ADD-14 does not apply and slot + 1 is element 3, sizeof(long) bytes on. A placement new there builds a long that slot[1] reads.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
int main() {
  unsigned char* bytes = static_cast<unsigned char*>(::operator new(8 * sizeof(long)));
  long* slot = static_cast<long*>(static_cast<void*>(bytes + 2 * sizeof(long)));
  long* made = ::new (slot + 1) long(31);
  int result = (made == slot + 1 && slot[1] == 31) ? 0 : 1;
  ::operator delete(bytes);
  return result;
}
