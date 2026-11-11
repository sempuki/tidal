// tidal-rules: ADD-11 STG-13
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: Roadmap round 11 programmer report, section 6 (the ADD-11 doubt, ledger row RM12-1). A long is built at a byte offset into storage from the allocation function. A cast of its address to unsigned char* points to a byte of the long (STG-13), and storage points into the storage, so the two pointers are not into the same array and inside - storage is undefined (ADD-11). No compiler or sanitizer reports the difference, so the matrix only builds the program. The defined path keeps the offset that made the address.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
int main() {
  unsigned char* storage = static_cast<unsigned char*>(::operator new(4 * sizeof(long)));
  long* made = ::new (storage + 2 * sizeof(long)) long(7);
  unsigned char* inside = reinterpret_cast<unsigned char*>(made);
  decltype(inside - storage) offset = inside - storage;
  ::operator delete(storage);
  return offset == 2 * static_cast<decltype(offset)>(sizeof(long)) ? 0 : 1;
}
