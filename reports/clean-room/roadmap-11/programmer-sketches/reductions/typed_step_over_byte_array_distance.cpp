// The byte distance of d + 1, where d is an int* cast from a byte array.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
alignas(int) unsigned char buf[4 * sizeof(int)];
int main() {
  int* d = static_cast<int*>(static_cast<void*>(buf));
  return reinterpret_cast<unsigned char*>(d + 1) - buf == sizeof(int) ? 0 : 1;
}
