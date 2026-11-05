struct S {
  union { char local[4]; char* heap; };
  S() {}
};
int main() {
  S s;
  s.local[0] = 'a';
  return s.local[0] == 'a' ? 0 : 1;
}
