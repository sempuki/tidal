// tidal-rules: ADD-8
// tidal-expect: run
int* volatile source = nullptr;
int main() {
  int* p = source;
  int* q = source;
  if (p - q != 0) return 1;
  if (p + 0 != nullptr) return 2;
  if (p - 0 != nullptr) return 3;
  return 0;
}
