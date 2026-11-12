const int& larger(const int& a, const int& b) { return a > b ? a : b; }
int main() {
  const int& r = larger(1, 2);  // both temporaries end with the full-expression
  return r == 2 ? 0 : 1;
}
