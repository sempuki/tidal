// tidal-rules: CAST-12
// tidal-expect: run
int target() { return 4; }
int main() {
  void* v = reinterpret_cast<void*>(&target);
  int (*back)() = reinterpret_cast<int (*)()>(v);
  return back() == 4 ? 0 : 1;
}
