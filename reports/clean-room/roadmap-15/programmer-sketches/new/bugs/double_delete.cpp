struct Node { int v; };
void release(Node* n) { delete n; }
int main() {
  Node* n = new Node{3};
  release(n);
  delete n;  // the second delete of the same object
  return 0;
}
