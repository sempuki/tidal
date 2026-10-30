// tidal-rules: NEW-2
// tidal-expect: run
int main(int argc, char**) {
  int n = argc - 1;
  int* a = new int[n];
  bool ok = a != nullptr;
  delete[] a;
  return ok ? 0 : 1;
}
