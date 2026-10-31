// ring.cpp: a fixed-capacity ring buffer of int.
// Spec rules used: CLS-3 (constexpr static member), CLS-5 (const member),
// CLS-15, OPR-2, OPR-3 (operator[] as a member), CX-1, CX-3 (constexpr),
// SA rules on static-assert.md, CE-1, CE-5 (changes to own objects),
// DCL-3 (reference return), CV-8.

class Ring {
public:
  static constexpr int capacity = 4;

  constexpr Ring() : head_(0), size_(0) {}

  constexpr int size() const { return size_; }
  constexpr bool empty() const { return size_ == 0; }
  constexpr bool full() const { return size_ == capacity; }

  // Adds at the back. Returns false if the buffer is full.
  constexpr bool push(int value) {
    if (full()) return false;
    data_[(head_ + size_) % capacity] = value;
    ++size_;
    return true;
  }

  // Removes from the front into out. Returns false if the buffer is empty.
  constexpr bool pop(int& out) {
    if (empty()) return false;
    out = data_[head_];
    head_ = (head_ + 1) % capacity;
    --size_;
    return true;
  }

  // Element i counts from the front. i must be less than size().
  constexpr int& operator[](int i) { return data_[(head_ + i) % capacity]; }
  constexpr int operator[](int i) const {
    return data_[(head_ + i) % capacity];
  }

private:
  // Each element needs a value before the object is copied or moved in
  // constant evaluation (CPY-12, OBJ-2), so all start at zero.
  int data_[capacity] = {};
  int head_;
  int size_;
};

static_assert(Ring::capacity == 4);

// Fills the ring past its capacity, so the indexes wrap around.
constexpr int wrap_test() {
  Ring r;
  for (int i = 1; i <= 4; ++i) r.push(i * 10);   // 10 20 30 40
  if (r.push(50)) return -1;                      // full
  int out = 0;
  r.pop(out);                                     // removes 10
  if (out != 10) return -2;
  r.push(50);                                     // 20 30 40 50, wrapped
  r[0] = 21;                                      // operator[] as lvalue
  return r[0] + r[3];                             // 21 + 50
}

constexpr int drain_test() {
  Ring r;
  r.push(7);
  int out = 0;
  int count = 0;
  while (r.pop(out)) ++count;
  if (r.pop(out)) return -1;                      // empty now
  return count * 100 + out;
}

constexpr Ring make_ring() {
  Ring r;
  r.push(3);
  r.push(4);
  return r;
}

static_assert(wrap_test() == 71);
static_assert(drain_test() == 107);

constexpr Ring fixed = make_ring();
static_assert(fixed.size() == 2);
static_assert(fixed[1] == 4);                     // const operator[]

int main() {
  // The same functions at run time (TR-5).
  if (wrap_test() != 71) return 1;
  if (drain_test() != 107) return 2;

  Ring r;
  for (int i = 0; i < 10; ++i) {
    if (r.full()) {
      int dropped = 0;
      r.pop(dropped);
    }
    r.push(i);
  }
  // The last four values remain: 6 7 8 9.
  if (r.size() != 4) return 3;
  for (int i = 0; i < r.size(); ++i)
    if (r[i] != 6 + i) return 4;
  return 0;
}
