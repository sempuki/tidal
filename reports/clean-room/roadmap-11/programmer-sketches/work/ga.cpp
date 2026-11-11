inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
alignas(int) unsigned char buf[4 * sizeof(int)];
int main() {
  int* d = static_cast<int*>(static_cast<void*>(buf));
  ::new (&d[0]) int(1);
  ::new (&d[1]) int(2);
  return d[0] + d[1] == 3 ? 0 : 1;
}
