// operator[] overloaded on const, returning a reference or a const reference.
struct Row {
  int v[3] = {1, 2, 3};
  int& operator[](int i) { return v[i]; }
  const int& operator[](int i) const { return v[i]; }
};
int sum(const Row& r) { return r[0] + r[1] + r[2]; }
int main() {
  Row r;
  r[1] = 20;
  return sum(r) == 24 ? 0 : 1;
}
