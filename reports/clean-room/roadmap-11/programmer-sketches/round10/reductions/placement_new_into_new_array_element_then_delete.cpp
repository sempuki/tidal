// Placement new of an int into an element of an array from new[], then
// delete[]. STG-9 makes the new int the element. The front end reports NEW-5.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
int main() {
  int* a = new int[2];
  ::new (static_cast<void*>(&a[0])) int(3);
  int r = a[0] == 3 ? 0 : 1;
  delete[] a;
  return r;
}
