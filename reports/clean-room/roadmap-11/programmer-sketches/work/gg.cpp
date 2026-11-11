inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
int main() {
  unsigned char* base = static_cast<unsigned char*>(::operator new(64));
  int* d = static_cast<int*>(static_cast<void*>(base + 8));
  ::new (d + 1) int(2);
  int r = d[1] == 2 ? 0 : 1;
  ::operator delete(base);
  return r;
}
