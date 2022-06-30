#include "unitary.hpp"

std::unique_ptr<Unitary1qBoxWrap> make_box(const std::array<std::array<std::array<double, 2>, 2>, 2> m) {
    Eigen::Matrix2cd eig;
    for (unsigned i = 0; i < 2; i++)
    {
        for(unsigned j=0; j < 2; j++) {
            std::complex c = {m[i][j][0], m[i][j][1]};
            eig << c;
        }
    }

    tket::Unitary1qBox u1q(eig);

    return std::make_unique<Unitary1qBoxWrap>(Unitary1qBoxWrap(u1q));
}