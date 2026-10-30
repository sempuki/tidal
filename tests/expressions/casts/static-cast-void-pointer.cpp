// tidal-rules: CAST-1
// tidal-expect: run
int main() {
  int x = 7;
  void* v = &x;
  int* p = static_cast<int*>(v);
  return *p == 7 ? 0 : 1;
}
