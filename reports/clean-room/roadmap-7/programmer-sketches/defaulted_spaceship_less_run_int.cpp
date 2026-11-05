namespace std {
struct strong_ordering {
    signed char v;
    static const strong_ordering less, equal, equivalent, greater;
    friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
    friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
struct P {
    int a, b;
    auto operator<=>(const P&) const = default;
};
static_assert(P{1, 2} < P{1, 3});
int main() {
    P x{1, 2}, y{1, 3};
    return x < y ? 0 : 1;
}
