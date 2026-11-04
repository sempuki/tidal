inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
int main() {
  alignas(int) unsigned char key[2][sizeof(int)];
  int* p = ::new (static_cast<void*>(key[0])) int(1);
  int* q = ::new (static_cast<void*>(key[1])) int(2);
  return *p + *q == 3 ? 0 : 1;
}
