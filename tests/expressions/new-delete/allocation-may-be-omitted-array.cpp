// tidal-rules: NEW-14
// tidal-expect: run
// tidal-note: The array forms. An implementation can skip the call for a new[] that is deleted at once.
using size = decltype(sizeof 0);
int calls = 0;
alignas(16) unsigned char heap[256];
void* operator new[](size) { ++calls; return heap; }
void operator delete[](void*) noexcept {}
void operator delete[](void*, size) noexcept {}
int main() {
  delete[] new int[4]{1, 2, 3, 4};
  return calls == 0 || calls == 1 ? 0 : 1;
}
