#include "unitary.hpp"
#include "Utils/Json.hpp"
#include "Circuit/Circuit.hpp"

std::unique_ptr<Unitary1qBoxWrap> make_box(const std::array<std::array<std::array<double, 2>, 2>, 2> m) {
    Eigen::Matrix2cd eig;
    for (unsigned i = 0; i < 2; i++)
    {
        for(unsigned j=0; j < 2; j++) {
            std::complex c = {m[i][j][0], m[i][j][1]};
            eig(i,j) = c;
        }
    }

    tket::Unitary1qBox u1q(eig);

    return std::make_unique<Unitary1qBoxWrap>(Unitary1qBoxWrap(u1q));
}

rust::String Unitary1qBoxWrap::circ_json() const{
    tket::Circuit c = *_tkbox.to_circuit();
    nlohmann::json j = c;
    return j.dump();
}
