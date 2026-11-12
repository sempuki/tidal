struct Adder {
  const int* base;
  int operator()(int x) const { return *base + x; }
};
Adder make(int start) {
  int local = start;
  return Adder{&local};  // the pointer outlives local
}
int main() {
  Adder a = make(5);
  return a(1) == 6 ? 0 : 1;
}
