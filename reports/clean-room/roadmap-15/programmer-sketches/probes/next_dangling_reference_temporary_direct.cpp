int main() {
  const int* p;
  { const int& r = 2; p = &r; }
  return *p == 2 ? 0 : 1;
}
