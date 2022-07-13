#pragma once

#include <Circuit/Boxes.hpp>
#include <array>
#include <complex>
#include <Eigen/Core>
#include "cxx.h"

class Unitary1qBoxWrap
{
public:
    tket::Unitary1qBox _tkbox;
    Unitary1qBoxWrap(tket::Unitary1qBox tkbox) : _tkbox(tkbox){};
    rust::String circ_json() const;
};

std::unique_ptr<Unitary1qBoxWrap> make_box(const std::array<std::array<std::array<double, 2>, 2>, 2> m);
