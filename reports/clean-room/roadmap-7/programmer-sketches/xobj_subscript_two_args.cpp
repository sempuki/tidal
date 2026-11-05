struct G {
    int cells[4]{};
    template <class Self>
    constexpr auto&& operator[](this Self&& self, int r, int c) {
        return static_cast<Self&&>(self).cells[r * 2 + c];
    }
};
int main() {
    G g{};
    g[1, 1] = 7;
    return g[1, 1] == 7 ? 0 : 1;
}
