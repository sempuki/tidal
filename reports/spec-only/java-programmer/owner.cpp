// owner.cpp: a class that owns a heap array of int.
// Spec rules used: NEW-2, NEW-4, NEW-5, NEW-9 (new[] / delete[]),
// CPY-1, CPY-2, CPY-3 (the four copy and move functions), CPY-4, CPY-5
// (what a user-declared function removes), CPY-9, CPY-11 (copy or move),
// DTOR-5, OBJ-3 (destruction order), VC-3 (static_cast<T&&> gives an xvalue).

class IntArray {
public:
  explicit IntArray(int size) : size_(size), data_(new int[size]()) {}

  ~IntArray() { delete[] data_; }   // delete[] of a null pointer is fine

  // Copy: a new array with the same values.
  IntArray(const IntArray& other)
      : size_(other.size_), data_(new int[other.size_]) {
    for (int i = 0; i < size_; ++i) data_[i] = other.data_[i];
  }

  IntArray& operator=(const IntArray& other) {
    if (this == &other) return *this;   // a = a must not free its own data
    int* fresh = new int[other.size_];
    for (int i = 0; i < other.size_; ++i) fresh[i] = other.data_[i];
    delete[] data_;
    data_ = fresh;
    size_ = other.size_;
    return *this;
  }

  // Move: take the array and leave the source empty but valid.
  IntArray(IntArray&& other) : size_(other.size_), data_(other.data_) {
    other.data_ = nullptr;
    other.size_ = 0;
  }

  IntArray& operator=(IntArray&& other) {
    if (this == &other) return *this;
    delete[] data_;
    data_ = other.data_;
    size_ = other.size_;
    other.data_ = nullptr;
    other.size_ = 0;
    return *this;
  }

  int size() const { return size_; }
  int& operator[](int i) { return data_[i]; }
  int operator[](int i) const { return data_[i]; }

private:
  int size_;
  int* data_;
};

// Returns a local by value. The local is moved, or is the result (CPY-12).
IntArray make_counting(int n) {
  IntArray result(n);
  for (int i = 0; i < n; ++i) result[i] = i;
  return result;
}

int sum(const IntArray& a) {
  int total = 0;
  for (int i = 0; i < a.size(); ++i) total += a[i];
  return total;
}

int main() {
  IntArray a = make_counting(5);          // 0 1 2 3 4
  if (sum(a) != 10) return 1;

  IntArray b = a;                         // copy constructor
  b[0] = 100;
  if (a[0] != 0) return 2;                // a is not changed
  if (b[0] != 100) return 3;

  IntArray c(2);
  c = a;                                  // copy assignment
  c[4] = 40;
  if (a[4] != 4 || c.size() != 5) return 4;

  IntArray& same = c;                     // c = c directly makes Clang warn
  c = same;                               // self-assignment is safe
  if (c[4] != 40) return 5;

  IntArray d = static_cast<IntArray&&>(b);   // move constructor (VC-3)
  if (d[0] != 100 || d.size() != 5) return 6;
  if (b.size() != 0) return 7;               // b is empty now

  IntArray e(1);
  e = static_cast<IntArray&&>(d);            // move assignment
  if (e[0] != 100 || d.size() != 0) return 8;

  b = a;                                  // a moved-from object can be reused
  if (sum(b) != 10) return 9;

  {
    IntArray gone(3);
    IntArray taker = static_cast<IntArray&&>(gone);
    if (taker.size() != 3) return 10;
  }                                       // both destroyed here, no double free
  return 0;
}
