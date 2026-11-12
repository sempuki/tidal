struct Node { int v; Node* next; int value() const { return v; } };
int main() {
  Node b{2, nullptr}, a{1, &b};
  int s = 0;
  for (Node* n = &a; ; n = n->next) {  // no end test
    s += n->value();
    if (s > 10) break;
  }
  return s;
}
