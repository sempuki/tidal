int&& pass(int&& x) { return x; }
int main() {
  int&& r = pass(5);
  return r == 5 ? 0 : 1;
}
