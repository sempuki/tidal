// tidal-rules: LOOP-1 JMP-3 ADD-9 UN-12 CX-3 RFOR-1
// tidal-expect: run
// tidal-note: Write suite W3, program search.cpp (reports/clean-room/W3/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Binary search over a sorted int array.
// Rules used: LOOP-1 (while), JMP-3 (return converts), ADD-9 (signed
// overflow is undefined, so mid = lo + (hi - lo) / 2), DCL-P3 (an array
// parameter is a pointer, so the length is passed separately), UN-12
// (sizeof of an array counts all elements), CX-3, RFOR-1.

struct Result {
  int index;   // position of key, or the position where it would go
  bool found;
};

// Half-open range [lo, hi). Go's sort.Search shape.
constexpr Result search(const int* a, int n, int key) {
  int lo = 0;
  int hi = n;
  while (lo < hi) {
    int mid = lo + (hi - lo) / 2;  // ADD-9: (lo + hi) can overflow
    if (a[mid] < key) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return {lo, lo < n && a[lo] == key};  // JMP-3: braced list initializes the result
}

constexpr int data[] = {-7, -2, 0, 3, 3, 8, 15, 21, 40};
constexpr int len = sizeof(data) / sizeof(data[0]);
static_assert(len == 9);

static_assert(search(data, len, -7).index == 0 && search(data, len, -7).found);
static_assert(search(data, len, 3).index == 3 && search(data, len, 3).found);
static_assert(search(data, len, 40).index == 8 && search(data, len, 40).found);
static_assert(search(data, len, 4).index == 5 && !search(data, len, 4).found);
static_assert(search(data, len, 99).index == 9 && !search(data, len, 99).found);
static_assert(search(data, len, -99).index == 0 && !search(data, len, -99).found);
static_assert(search(data, 0, 1).index == 0 && !search(data, 0, 1).found);

constexpr bool sorted(const int* a, int n) {
  for (int i = 1; i < n; ++i) {
    if (a[i - 1] > a[i]) return false;
  }
  return true;
}
static_assert(sorted(data, len));

int main() {
  int squares[100];
  int k = 0;
  for (int& s : squares) {  // RFOR-1: int& changes the elements
    s = k * k;
    ++k;
  }
  int n = sizeof(squares) / sizeof(squares[0]);
  for (int i = 0; i < n; ++i) {
    Result r = search(squares, n, i * i);
    if (!r.found || r.index != i) return 1;
    if (i > 0) {
      Result miss = search(squares, n, i * i - 1);
      if (miss.found && i * i - 1 != (i - 1) * (i - 1)) return 2;
    }
  }
  if (search(squares, n, 10000).found) return 3;
  return 0;
}
