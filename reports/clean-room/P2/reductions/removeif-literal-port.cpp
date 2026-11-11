struct Node { int value; Node* next; };
int main() {
  Node* head = new Node{1, new Node{2, nullptr}};
  int seen = 0;
  for (Node* n = head; n != nullptr; n = n->next) {  // Java habit: n->next after delete
    seen += n->value;
    delete n;
  }
  return seen == 3 ? 0 : 1;
}
