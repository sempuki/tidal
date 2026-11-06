// tidal-rules: STG-8 STG-23
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: misaligned address
// tidal-note: Decision 0059, item F2 (X23-6), with the round 9 writer probe x23-6-underaligned-implicit-object.cpp. A call of the global allocation function creates implicit objects, but each object is aligned for its type. At an address that is 16 more than a multiple of 64, no object of a 64-aligned class exists, so the write through the pointer has undefined behavior. The sanitizer stops it.
struct alignas(64) Line { int x; };
int main() {
  void* raw = ::operator new(4 * sizeof(Line));
  char* at = static_cast<char*>(raw);
  while (reinterpret_cast<decltype(sizeof 0)>(at) % 64 != 16)
    at += 16;
  Line* p = reinterpret_cast<Line*>(at);
  p->x = 1;
  int v = p->x;
  ::operator delete(raw);
  return v == 1 ? 0 : 1;
}
