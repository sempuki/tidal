namespace std { using size_t = decltype(sizeof 0); }
void* operator new(std::size_t, void* p) noexcept;
struct Box {
    union { int v; };
    Box() { ::new (&v) int(3); }
};
int main() {
    Box b;
    return b.v == 3 ? 0 : 1;
}
