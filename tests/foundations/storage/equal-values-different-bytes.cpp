// tidal-rules: STG-11
// tidal-expect: run
int main() {
  double positive = 0.0;
  double negative = -0.0;
  const unsigned char* a = reinterpret_cast<const unsigned char*>(&positive);
  const unsigned char* b = reinterpret_cast<const unsigned char*>(&negative);
  bool same_bytes = true;
  for (unsigned i = 0; i != sizeof(double); ++i)
    if (a[i] != b[i]) same_bytes = false;
  return positive == negative && !same_bytes ? 0 : 1;
}
