struct Box { int v[2]{}; };
int main() {
    Box b{{3, 4}};
    auto copy = auto(b);
    copy.v[0] *= 2;
    return copy.v[0] == 6 && b.v[0] == 3 ? 0 : 1;
}
