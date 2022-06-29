use autocxx::include_cpp;

// pub mod tkbind {
include_cpp! {
    //#include "Circuit/Circuit.hpp"
    #include "Circuit/Command.hpp"
    #include "Circuit/Boxes.hpp"
    #include "Ops/Op.hpp"
    //#include "Ops/MetaOp.hpp"
    #include "Utils/UnitID.hpp"
    // #include "tklog/TketLog.hpp"

    safety!(unsafe)

    generate_pod!("tket::UnitType")
    generate!("tket::InvalidUnitConversion")
    generate!("tket::UnitID")
    generate!("tket::Qubit")
    generate!("tket::Bit")
    generate!("tket::Node")

    // generate!("tket::LogPtr_t")

    //generate!("tket::Architecture")
    //generate!("tket::FullyConnected")
    //generate!("tket::RingArch")

    //generate!("tket::Circuit")

    generate_pod!("tket::OpType")
    generate_pod!("tket::EdgeType")
    // generate!("tket::PauliExpBox")
    //generate!("tket::MetaOp")
}

// pub ffi::Qubit;
pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let fc = ffi::tket::FullConnected;
        let reg = autocxx::c_uint(1);
        let qubit = ffi::tket::Qubit::new1(reg);
        // https://github.com/google/autocxx/issues/197
        //qubit.pin_mut().repr();
    }
}
