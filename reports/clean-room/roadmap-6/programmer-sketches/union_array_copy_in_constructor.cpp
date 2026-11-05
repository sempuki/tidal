struct S {
  union { char local[4]; char* heap; };
  S() : local{} {}
  S(const S& o) {
    for (int i = 0; i < 4; ++i) local[i] = o.local[i];
  }
};
int main() {
  S a;
  S b(a);
  return b.local[0] == 0 ? 0 : 1;
}
