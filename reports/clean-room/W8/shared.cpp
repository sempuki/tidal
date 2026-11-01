// A shared owner with a count, written from the Tidal specification.
// Rules used: CPY-P1, CPY-P2, CPY-1, CPY-2, CPY-3, CPY-9, CPY-11,
// NEW-1, NEW-4, NEW-9, DTOR-5, CTOR-13 (member order).

int live = 0;

struct Node {
  int value;
  explicit Node(int v) : value(v) { ++live; }
  ~Node() { --live; }
};

template <class T>
class Shared {
  // One block per owned object. Every owner points to the same block.
  struct Block {
    T* object;
    long count;
  };
  Block* block = nullptr;

  void drop() {
    if (block != nullptr) {
      --block->count;
      if (block->count == 0) {
        delete block->object;
        delete block;
      }
      block = nullptr;
    }
  }

public:
  Shared() = default;
  explicit Shared(T* p) : block(p ? new Block{p, 1} : nullptr) {}

  Shared(const Shared& other) : block(other.block) {
    if (block != nullptr) ++block->count;
  }
  Shared(Shared&& other) noexcept : block(other.block) { other.block = nullptr; }

  Shared& operator=(const Shared& other) {
    // Count the new owner first, so a self-assignment keeps the object.
    // other can be *this, and drop() clears block, so keep the pointer.
    Block* incoming = other.block;
    if (incoming != nullptr) ++incoming->count;
    drop();
    block = incoming;
    return *this;
  }
  Shared& operator=(Shared&& other) noexcept {
    if (this != &other) {
      drop();
      block = other.block;
      other.block = nullptr;
    }
    return *this;
  }

  ~Shared() { drop(); }

  long use_count() const { return block ? block->count : 0; }
  T* get() const { return block ? block->object : nullptr; }
  T& operator*() const { return *block->object; }
  void reset() { drop(); }
};

Shared<Node> pass_through(Shared<Node> p) {
  return p;                       // a parameter is moved to the result (CPY-11)
}

int main() {
  {
    Shared<Node> a(new Node(5));
    if (a.use_count() != 1) return 1;
    {
      Shared<Node> b = a;         // copy (CPY-9): count 2
      if (a.use_count() != 2 || b.use_count() != 2) return 2;
      Shared<Node> c;
      c = b;                      // copy assignment: count 3
      if (a.use_count() != 3) return 3;
      (*c).value = 9;
      if ((*a).value != 9) return 4;
      Shared<Node> d = static_cast<Shared<Node>&&>(c);  // move: count stays 3
      if (c.get() != nullptr || d.use_count() != 3) return 5;
      Shared<Node>& same = d;
      d = same;                   // self-assignment: count stays 3
      if (d.use_count() != 3) return 6;
    }                             // b and d end: count 1
    if (a.use_count() != 1) return 7;
    if (live != 1) return 8;

    Shared<Node> e = pass_through(a);    // a copied into p, p moved out
    if (a.use_count() != 2) return 9;

    Shared<Node> other(new Node(1));
    if (live != 2) return 10;
    other = e;                    // Node(1) has no owner left
    if (live != 1) return 11;
    if (a.use_count() != 3) return 12;

    a.reset();
    e.reset();
    if (live != 1) return 13;     // other still owns Node(9)
    if (other.use_count() != 1 || (*other).value != 9) return 14;
  }
  if (live != 0) return 15;
  return 0;
}
