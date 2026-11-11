inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
int main() {
  int* d = static_cast<int*>(::operator new(4 * sizeof(int)));
  ::new (d + 1) int(2);
  int r = d[1] == 2 ? 0 : 1;
  ::operator delete(d);
  return r;
}
