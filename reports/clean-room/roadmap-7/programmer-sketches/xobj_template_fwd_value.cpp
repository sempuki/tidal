struct G {
    int v = 0;
    template <class Self>
    int get(this Self&& self) { return self.v; }
};
int main() {
    G g{};
    g.v = 7;
    return g.get() == 7 ? 0 : 1;
}
