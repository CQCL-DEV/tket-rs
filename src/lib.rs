use autocxx::include_cpp;
// pub mod tkbind {
include_cpp! {
    //#include "Circuit/Circuit.hpp"
    #include "Circuit/Command.hpp"
    #include "Circuit/Boxes.hpp"
    #include "Ops/Op.hpp"
    //#include "Ops/MetaOp.hpp"
    #include "Utils/UnitID.hpp"
    #include "unitary.hpp"
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
    // generate!("Unitary1qBoxWrap")
    // generate!("make_box")
    // generate!("tket::Unitary1qBox")
    //generate!("tket::MetaOp")
    extern_cpp_opaque_type!("Unitary1qBoxWrap", ffi2::Unitary1qBoxWrap)
}

// pub ffi::Qubit;
pub use ffi::*;

#[cxx::bridge]
mod ffi2 {
    unsafe extern "C++" {
        include!("unitary.hpp");

        type Unitary1qBoxWrap;

        fn make_box(m: [[[f64; 2]; 2]; 2]) -> UniquePtr<Unitary1qBoxWrap>;
        fn circ_json(self: &Unitary1qBoxWrap) -> String;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let fc = ffi::tket::FullConnected;

        // let reg = autocxx::c_uint(1);
        // let qubit = ffi::tket::Qubit::new1(reg);

        // https://github.com/google/autocxx/issues/197
        //qubit.pin_mut().repr();

        let box2 = ffi2::make_box([[[0.0, 0.0], [1., 0.]], [[1., 0.], [0., 0.]]]);
        println!("{}", box2.circ_json());
    }
}
