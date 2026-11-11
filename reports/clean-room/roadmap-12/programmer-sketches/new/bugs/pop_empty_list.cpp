// Planted bug: pop_front on an empty intrusive list reads through a null head.
struct Node { int v; Node* next; };
struct List { Node* head = nullptr; int pop() { Node* h = head; head = h->next; return h->v; } };
int main() { Node n{1, nullptr}; List l{&n}; l.pop(); return l.pop(); }
