// tidal-rules: VC-16
// tidal-expect: run
struct CopyFails {
  CopyFails() {}
  CopyFails(const CopyFails&) { throw 1; }
};
struct EndFails {
  ~EndFails() noexcept(false) { throw 2; }
};
void take(CopyFails) noexcept {}
void keep(EndFails) noexcept {}
int main() {
  int caught = 0;
  CopyFails source;
  try { take(source); } catch (int v) { caught += v; }
  try { keep(EndFails{}); } catch (int v) { caught += v * 10; }
  return caught == 21 ? 0 : 1;
}
