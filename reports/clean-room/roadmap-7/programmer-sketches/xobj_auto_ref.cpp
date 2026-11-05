struct G {
    int v = 0;
    int& get(this auto& self) { return self.v; }
};
int main() {
    G g{};
    g.get() = 7;
    return g.v == 7 ? 0 : 1;
}
