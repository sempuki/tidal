struct Box { int v[2]{}; };
template <class G>
G doubled(const G& g) {
    auto copy = auto(g);
    copy.v[0] *= 2;
    return copy;
}
int main() {
    Box b{{3, 4}};
    return doubled(b).v[0] == 6 && b.v[0] == 3 ? 0 : 1;
}
