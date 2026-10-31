// tidal-rules: ALN-1
// tidal-expect: run
unsigned long address(const void* p) { return reinterpret_cast<unsigned long>(p); }
alignas(32) char buffer[10];
int main() { alignas(16) int local = 0; return address(buffer) % 32 == 0 && address(&local) % 16 == 0 ? 0 : 1; }
