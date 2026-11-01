// tidal-rules: NEW-2
// tidal-expect: run
// tidal-note: The size has a class type with one conversion function to an integer type. The conversion gives the number of elements.
struct Two { operator int() const { return 2; } };
int main() {
  int* p = new int[Two{}];
  p[1] = 0;
  delete[] p;
  return 0;
}
