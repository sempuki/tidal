struct G {
    int cells[4]{};
    template <class Self>
    constexpr auto&& at(this Self&& self, int i) {
        return static_cast<Self&&>(self).cells[i];
    }
};
int main() {
    G g{};
    g.at(3) = 7;
    const G& c = g;
    return c.at(3) == 7 ? 0 : 1;
}
