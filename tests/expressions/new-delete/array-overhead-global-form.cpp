// tidal-rules: NEW-34
// tidal-expect: run
// tidal-note: The replaced global array form, with a class that has a destructor. The request covers the elements, and the array lies inside the storage.
using size = decltype(sizeof 0);
alignas(16) unsigned char buffer[256];
size requested = 0;
void* operator new[](size n) { requested = n; return buffer; }
void operator delete[](void*) noexcept {}
void operator delete[](void*, size) noexcept {}
struct Counted { short value = 5; ~Counted() {} };
int main() {
  Counted* items = new Counted[4];
  if (requested < 4 * sizeof(Counted)) return 1;
  unsigned char* first = reinterpret_cast<unsigned char*>(items);
  if (first < buffer || first + 4 * sizeof(Counted) > buffer + requested) return 2;
  if (items[3].value != 5) return 3;
  delete[] items;
  return 0;
}
