// Planted bug: the loop frees a node and then reads its next pointer.
struct Node { int v; Node* next; };
int main() {
  Node* head = new Node{1, new Node{2, new Node{3, nullptr}}};
  int sum = 0;
  for (Node* n = head; n; n = n->next) { sum += n->v; delete n; }
  return sum == 6 ? 0 : 1;
}
