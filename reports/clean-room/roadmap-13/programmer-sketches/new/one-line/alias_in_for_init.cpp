// An alias declaration as the init statement of a range-based for loop.
int main() {
  long a[] = {1, 2, 3};
  int s = 0;
  for (using T = long; T i : a) s += static_cast<int>(i);
  return s - 6;
}
