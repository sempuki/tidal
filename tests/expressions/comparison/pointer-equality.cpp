// tidal-rules: CMP-15
// tidal-expect: run
int a, b;
int main() {
  int* p = &a; int* q = &a; int* r = &b; int* z = nullptr;
  return p == q && p != r && z == nullptr && p != nullptr && z == 0 ? 0 : 1;
}
