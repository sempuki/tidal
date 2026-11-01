// tidal-rules: NEW-34
// tidal-expect: run
using size = decltype(sizeof 0);
alignas(16) unsigned char buffer[256];
size requested = 0;
void* operator new[](size n, int) { requested = n; return buffer; }
struct Tracked { int value = 7; ~Tracked() { value = 0; } };
int main() {
  Tracked* objects = new (1) Tracked[3];
  if (requested < 3 * sizeof(Tracked)) return 1;
  unsigned char* first = reinterpret_cast<unsigned char*>(objects);
  if (first < buffer || first + 3 * sizeof(Tracked) > buffer + requested) return 2;
  if (objects[2].value != 7) return 3;
  return 0;
}
