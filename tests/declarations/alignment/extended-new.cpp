// tidal-rules: ALN-4
// tidal-expect: run
unsigned long address(const void* p) { return reinterpret_cast<unsigned long>(p); }
struct alignas(256) Block { int v; };
int main() { Block* b = new Block; bool ok = address(b) % 256 == 0; delete b; Block* a = new Block[3]; ok = ok && address(a) % 256 == 0; delete[] a; return ok ? 0 : 1; }
