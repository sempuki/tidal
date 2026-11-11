// tidal-rules: STG-29 ADD-14
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction placement_at_typed_step_into_byte_array.cpp. Q-0430. The cast of the byte array's address gives an int* that points to a byte (STG-29), so ADD-14 counts that byte as an array of one int, and first + 1 is one past it. The provisional answer places that position sizeof(int) bytes after first, as every compiler does. A placement new at the position builds an int in the array, and the program reads both ints through the pointers that placement new returns.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
alignas(int) unsigned char pool[4 * sizeof(int)];
int main() {
  int* first = static_cast<int*>(static_cast<void*>(pool));
  int* a = ::new (static_cast<void*>(first)) int(5);
  int* b = ::new (static_cast<void*>(first + 1)) int(6);
  return (*a == 5 && *b == 6) ? 0 : 1;
}
