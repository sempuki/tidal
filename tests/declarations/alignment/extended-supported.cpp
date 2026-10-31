// tidal-rules: ALN-4
// tidal-expect: run
unsigned long address(const void* p) { return reinterpret_cast<unsigned long>(p); }
struct alignas(4096) Page { char data[16]; };
Page global_page;
int main() { Page local; return address(&global_page) % 4096 == 0 && address(&local) % 4096 == 0 ? 0 : 1; }
