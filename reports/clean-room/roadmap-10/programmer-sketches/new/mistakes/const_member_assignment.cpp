// Planted mistake: copy assignment of a class with a const member.
struct Id { const int v; };
int main() { Id a{1}, b{2}; a = b; return a.v; }
