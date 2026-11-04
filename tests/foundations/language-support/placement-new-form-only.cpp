// tidal-rules: LSUP-7 NEW-24
// tidal-expect: run
// tidal-note: The program defines the placement operator new and no placement operator delete. The constructor throws, no deallocation function runs, and the byte array can host a new object.
using size = decltype(sizeof 0);
inline void* operator new(size, void* at) noexcept { return at; }
struct Gadget {
  int value;
  explicit Gadget(int v) : value(v) { if (v < 0) throw v; }
};
alignas(Gadget) unsigned char bytes[sizeof(Gadget)];
int main() {
  int caught = 0;
  try { new (bytes) Gadget(-1); } catch (int v) { caught = v; }
  Gadget* g = new (bytes) Gadget(7);
  return caught == -1 && g->value == 7 ? 0 : 1;
}
