struct G {
    int cells[4]{};
    template <class Self>
    constexpr auto&& operator[](this Self&& self, int i) {
        return static_cast<Self&&>(self).cells[i];
    }
};
int main() {
    G g{};
    g[3] = 7;
    return g[3] == 7 ? 0 : 1;
}
