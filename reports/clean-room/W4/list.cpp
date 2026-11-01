// W4: a singly linked list of int with insert and remove.
// Rules: NEW-1, NEW-9, NEW-4, NEW-5, OBJ-5, PFX-10, DCL-1, DCL-2, OBJ-2.

struct Node {
  int value;
  Node* next;
};

class List {
  Node* head = nullptr;
  int count = 0;

public:
  List() = default;
  List(const List&) = delete;             // two lists must not own one chain
  List& operator=(const List&) = delete;
  ~List() { clear(); }

  void push_front(int v) {
    head = new Node{v, head};             // NEW-1
    ++count;
  }

  // Insert v so that the list stays in ascending order.
  void insert_sorted(int v) {
    Node** link = &head;                  // DCL-2: a pointer to a pointer
    while (*link != nullptr && (*link)->value < v) {
      link = &(*link)->next;
    }
    *link = new Node{v, *link};
    ++count;
  }

  // Remove the first node that holds v. Returns true if a node was removed.
  bool remove(int v) {
    Node** link = &head;
    while (*link != nullptr) {
      Node* n = *link;
      if (n->value == v) {
        *link = n->next;                  // unlink before delete
        delete n;                         // NEW-9; n is now invalid (OBJ-2)
        --count;
        return true;
      }
      link = &n->next;
    }
    return false;
  }

  void clear() {
    Node* n = head;
    while (n != nullptr) {
      Node* next = n->next;               // read next before delete
      delete n;
      n = next;
    }
    head = nullptr;
    count = 0;
  }

  int size() const { return count; }

  int at(int index) const {
    Node* n = head;
    for (int i = 0; i < index; ++i) n = n->next;
    return n->value;
  }

  bool contains(int v) const {
    for (Node* n = head; n != nullptr; n = n->next)
      if (n->value == v) return true;
    return false;
  }
};

int main() {
  List list;
  list.insert_sorted(5);
  list.insert_sorted(1);
  list.insert_sorted(3);
  list.insert_sorted(4);
  list.insert_sorted(2);
  if (list.size() != 5) return 1;
  for (int i = 0; i < 5; ++i)
    if (list.at(i) != i + 1) return 2;

  if (!list.remove(3)) return 3;          // middle
  if (!list.remove(1)) return 4;          // head
  if (!list.remove(5)) return 5;          // tail
  if (list.remove(9)) return 6;           // absent
  if (list.size() != 2) return 7;
  if (list.at(0) != 2 || list.at(1) != 4) return 8;
  if (list.contains(3)) return 9;

  list.push_front(0);
  if (list.at(0) != 0 || list.size() != 3) return 10;

  List* heap = new List;                  // OBJ-5: lives until delete
  heap->push_front(7);
  heap->push_front(8);
  delete heap;                            // runs ~List, which frees both nodes

  Node* none = nullptr;
  delete none;                            // NEW-4: does nothing
  return 0;
}
