#include "registry.h"

struct Totals {
    int count;
    int checksum;
    auto operator<=>(const Totals&) const = default;
};

namespace std {
struct strong_ordering {
    signed char v;
    static const strong_ordering less, equal, equivalent, greater;
    friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
    friend constexpr bool operator==(strong_ordering a, decltype(nullptr)) { return a.v == 0; }
    friend constexpr bool operator<(strong_ordering a, decltype(nullptr)) { return a.v < 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

int main() {
    using namespace app;
    const Registry& reg = Registry::get();
    if (reg.size() != 3 || registered != 3) return 1;
    Totals t{0, 0};
    for (const auto& [name, kind, fn] : reg) {
        ++t.count;
        t.checksum += fn(6, 7) + static_cast<int>(kind);
        if (kind_name(kind)[0] != name[0]) return 2;
    }
    if (t != Totals{3, 13 + 0 + 42 + 1 + (-6) + 2}) return 3;
    if (t < Totals{3, 0}) return 4;
    const Command* mul = reg.find("mul");
    if (!mul || fold_apply(mul->fn, 1, 2, 3, 4) != 24) return 5;
    if (fold_apply(mul->fn, 9) != 9) return 6;
    if (reg.find("div") != nullptr) return 7;
    return 0;
}
