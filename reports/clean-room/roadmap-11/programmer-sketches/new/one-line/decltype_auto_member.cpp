// decltype(auto) that returns a member by reference through parentheses.
struct Box { int v = 1; decltype(auto) ref() { return (v); } decltype(auto) val() { return v; } };
int main() { Box b; b.ref() = 5; int& r = b.ref(); r += 1; return b.val() == 6 ? 0 : 1; }
