// A command registry shared by two units.
#ifndef REGISTRY_H
#define REGISTRY_H

namespace app {

enum class Kind : unsigned char { add, mul, neg };

struct Command {
    const char* name;
    Kind kind;
    int (*fn)(int, int);
};

class Registry {
    Command items_[8]{};
    int n_ = 0;
    Registry() = default;
public:
    static Registry& get() {
        static Registry r;          // built on first use
        return r;
    }
    Registry(const Registry&) = delete;
    bool add(Command c) {
        if (n_ == 8) return false;
        items_[n_++] = c;
        return true;
    }
    int size() const { return n_; }
    const Command* begin() const { return items_; }
    const Command* end() const { return items_ + n_; }
    const Command* find(const char* name) const;
};

inline int registered = 0;          // one variable in the program

// Registers at static initialization of the unit that defines it.
struct AutoRegister {
    explicit AutoRegister(Command c) {
        if (Registry::get().add(c)) ++registered;
    }
};

template <class T>
T fold_apply(int (*fn)(int, int), T first);

template <class T, class... Rest>
T fold_apply(int (*fn)(int, int), T first, Rest... rest) {
    T acc = first;
    ((acc = fn(acc, rest)), ...);
    return acc;
}

extern template int fold_apply<int>(int (*)(int, int), int);

const char* kind_name(Kind k);

}  // namespace app

#endif
