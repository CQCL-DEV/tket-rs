use autocxx::include_cpp;

include_cpp! {
    #include "Utils/UnitID.hpp"
    #include "Graphs/AdjacencyData.hpp"
    safety!(unsafe)
    generate_pod!("tket::UnitType")
    generate!("tket::graphs::AdjacencyData")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let unit_type = ffi::tket::UnitType::Qubit;
        let data = ffi::tket::graphs::AdjacencyData;
    }
}
