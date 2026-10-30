// tidal-rules: CAST-6
// tidal-expect: run
int main() {
  int x = 3;
  unsigned long long n = reinterpret_cast<unsigned long long>(&x);
  int* p = reinterpret_cast<int*>(n);
  return p == &x && *p == 3 ? 0 : 1;
}
