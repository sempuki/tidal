// tidal-rules: NEW-15
// tidal-expect: run
// tidal-note: The array form of delete, with a const class object whose conversion function gives a pointer to the first element.
struct Buffer {
  double* data;
  operator double*() const { return data; }
};
int main() {
  const Buffer b{new double[3]{1.0, 2.0, 3.0}};
  if (b.data[2] != 3.0) return 1;
  delete[] b;
  return 0;
}
