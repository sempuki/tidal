const int& pick(const int& a) { return a; }
int main() {
  int two = 2;
  const int& r = pick(two);
  return r == 2 ? 0 : 1;
}
