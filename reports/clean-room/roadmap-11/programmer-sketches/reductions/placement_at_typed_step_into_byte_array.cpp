// A T* cast from a byte array, stepped to one past the byte, then
// placement new there. From arena_second_try.cpp.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
alignas(int) unsigned char buf[4 * sizeof(int)];
int main() {
  int* d = static_cast<int*>(static_cast<void*>(buf));
  ::new (d + 1) int(2);
  return d[1] == 2 ? 0 : 1;
}
