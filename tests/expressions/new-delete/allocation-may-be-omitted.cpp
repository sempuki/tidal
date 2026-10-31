// tidal-rules: NEW-14
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int calls = 0;
alignas(16) unsigned char heap[256];
void* operator new(decltype(sizeof 0) n) { ++calls; static decltype(sizeof 0) used = 0; void* p = heap + used; used += (n + 15) / 16 * 16; return p; }
void operator delete(void*) noexcept {}
void operator delete(void*, decltype(sizeof 0)) noexcept {}
int main() { delete new int(1); return calls == 0 || calls == 1 ? 0 : 1; }
