// A lower bound by binary search over a sorted array, written with pointers.
const int* lower_bound(const int* first, const int* last, int value) {
  while (first < last) {
    const int* mid = first + (last - first) / 2;
    if (*mid < value) first = mid + 1; else last = mid;
  }
  return first;
}
int main() {
  int a[] = {1, 3, 3, 5, 8, 13};
  const int* end = a + 6;
  return lower_bound(a, end, 3) == a + 1 && lower_bound(a, end, 4) == a + 3 && lower_bound(a, end, 99) == end ? 0 : 1;
}
