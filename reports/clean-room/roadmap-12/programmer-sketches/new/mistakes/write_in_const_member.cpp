// Planted mistake: a const member function assigns to a member.
struct Counter { int hits = 0; int touch() const { return ++hits; } };
int main() { Counter c; return c.touch() - 1; }
