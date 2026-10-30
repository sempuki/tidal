// tidal-rules: NEW-4
// tidal-expect: run
int main() {
  int* p = nullptr;
  delete p;
  int* q = nullptr;
  delete[] q;
  return 0;
}
