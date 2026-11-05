struct S {
  union { char local[4]; char* heap; };
  S() {}
  void set() { local[1] = 'x'; }
};
int main() {
  S b;
  b.set();
  return b.local[1] == 'x' ? 0 : 1;
}
