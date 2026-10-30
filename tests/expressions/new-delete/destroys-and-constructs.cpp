// tidal-rules: NEW-9
// tidal-expect: run
int built = 0, destroyed = 0;
struct S { S() { ++built; } ~S() { ++destroyed; } };
int main() {
  S* one = new S;
  S* many = new S[3];
  int after_new = built;
  delete one;
  delete[] many;
  return after_new == 4 && destroyed == 4 ? 0 : 1;
}
