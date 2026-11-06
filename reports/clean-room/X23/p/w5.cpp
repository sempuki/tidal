// No class functions: the implicit global aligned forms always serve an over-aligned type.
struct alignas(64) Wide { int v; };
int main() {
  Wide* w = new Wide{1};
  Wide* a = new Wide[3];
  int ok = reinterpret_cast<decltype(sizeof 0)>(w) % 64 == 0 && reinterpret_cast<decltype(sizeof 0)>(a) % 64 == 0;
  delete w; delete[] a;
  return ok ? 0 : 1;
}
