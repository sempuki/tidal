// An explicit constructor used by copy-list-initialization.
struct Meters { explicit Meters(int v) : v(v) {} int v; };
Meters make() { return {5}; }
int main() { return make().v - 5; }
