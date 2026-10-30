// tidal-rules: REF-2
// tidal-expect: run
int main() {
  long l = 1;
  const int& r = l;
  l = 2;
  return r == 1 && static_cast<const void*>(&r) != static_cast<const void*>(&l) ? 0 : 1;
}
