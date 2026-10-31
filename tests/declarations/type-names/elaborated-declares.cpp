// tidal-rules: TYN-4
// tidal-expect: run
struct Node* make();
struct Node { int v; };
Node* make() { static Node n{5}; return &n; }
int main() { return make()->v == 5 ? 0 : 1; }
