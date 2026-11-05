struct G {
    int cells[4]{};
    int& operator[](int r, int c) { return cells[r * 2 + c]; }
};
int main() {
    G g{};
    g[1, 1] = 7;
    return g[1, 1] == 7 ? 0 : 1;
}
