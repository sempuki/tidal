// An explicit constructor is used in copy-initialization.
struct Id { explicit Id(int v) : v(v) {} int v; };
Id make() { return 7; }
int main() { return make().v - 7; }
