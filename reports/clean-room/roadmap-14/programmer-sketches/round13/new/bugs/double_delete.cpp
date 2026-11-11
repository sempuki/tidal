// An object is deleted twice through two owners.
struct Node { int v; };
int main() { Node* a = new Node{1}; Node* b = a; delete a; delete b; return 0; }
