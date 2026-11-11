// A doubly linked list with an iterator, a reverse walk, and removal of
// the elements that match a predicate. Port of Chain.java, with no
// #include.

template <class T> struct Node {
  T value;
  Node* prev = nullptr;
  Node* next = nullptr;
  Node(const T& v) : value(v) {}
};

template <class T> class LinkedSeq {
  Node<T>* head = nullptr;
  Node<T>* tail = nullptr;
  int count = 0;

public:
  // The iterator that a range-based for uses (RFOR-2): prefix ++,
  // a comparison with end by !=, and * for the element.
  class Iterator {
    Node<T>* at;
  public:
    explicit Iterator(Node<T>* n) : at(n) {}
    T& operator*() const { return at->value; }
    Iterator& operator++() { at = at->next; return *this; }
    bool operator==(const Iterator& other) const { return at == other.at; }
  };

  LinkedSeq() = default;
  LinkedSeq(const LinkedSeq&) = delete;
  LinkedSeq& operator=(const LinkedSeq&) = delete;
  ~LinkedSeq() {
    Node<T>* n = head;
    while (n != nullptr) {
      Node<T>* next = n->next;
      delete n;
      n = next;
    }
  }

  void addLast(const T& value) {
    Node<T>* n = new Node<T>(value);
    if (tail == nullptr) { head = tail = n; }
    else { tail->next = n; n->prev = tail; tail = n; }
    count++;
  }

  void addFirst(const T& value) {
    Node<T>* n = new Node<T>(value);
    if (head == nullptr) { head = tail = n; }
    else { n->next = head; head->prev = n; head = n; }
    count++;
  }

  // The predicate is any callable, such as a lambda (LAM-15, TPL-P5).
  template <class Pred> int removeIf(Pred p) {
    int removed = 0;
    Node<T>* n = head;
    while (n != nullptr) {
      Node<T>* next = n->next;           // read before the delete
      if (p(n->value)) {
        if (n->prev != nullptr) n->prev->next = n->next; else head = n->next;
        if (n->next != nullptr) n->next->prev = n->prev; else tail = n->prev;
        delete n;
        count--;
        removed++;
      }
      n = next;
    }
    return removed;
  }

  int size() const { return count; }

  Iterator begin() const { return Iterator(head); }
  Iterator end() const { return Iterator(nullptr); }

  // Java returns null when nothing matches. Here a null pointer does.
  template <class Pred> const T* lastMatching(Pred p) const {
    for (Node<T>* n = tail; n != nullptr; n = n->prev)
      if (p(n->value)) return &n->value;
    return nullptr;
  }
};

// Appends the decimal digits of v and a space, as StringBuilder.append.
static int appendNumber(char* buf, int len, int v) {
  char digits[12];
  int d = 0;
  bool negative = v < 0;
  unsigned u = negative ? 0u - (unsigned)v : (unsigned)v;
  do { digits[d++] = (char)('0' + u % 10u); u /= 10u; } while (u != 0u);
  if (negative) buf[len++] = '-';
  while (d > 0) buf[len++] = digits[--d];
  buf[len++] = ' ';
  return len;
}

static bool sameText(const char* a, const char* b) {
  while (*a != '\0' && *a == *b) { ++a; ++b; }
  return *a == *b;
}

int main() {
  LinkedSeq<int> seq;
  for (int i = 1; i <= 6; i++) seq.addLast(i * 10);
  seq.addFirst(5);
  int sum = 0;
  for (int v : seq) sum += v;
  int failures = 0;
  if (sum != 215) failures++;
  if (seq.removeIf([](int v) { return v % 20 == 0; }) != 3) failures++;
  if (seq.size() != 4) failures++;
  char order[64];
  int len = 0;
  for (int v : seq) len = appendNumber(order, len, v);
  order[len] = '\0';
  if (!sameText(order, "5 10 30 50 ")) failures++;
  const int* last = seq.lastMatching([](int v) { return v < 40; });
  if (last == nullptr || *last != 30) failures++;
  return failures;
}
