struct S {
  union { char local[4]; char* heap; } u;
  S() {}
  void set() { u.local[1] = 'x'; }
};
int main() {
  S b;
  b.set();
  return b.u.local[1] == 'x' ? 0 : 1;
}
