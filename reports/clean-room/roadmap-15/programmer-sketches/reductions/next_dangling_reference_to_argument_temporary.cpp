const int& pick(const int& a) { return a; }
int main() {
  const int& r = pick(2);  // the temporary ends with the full-expression
  return r == 2 ? 0 : 1;
}
