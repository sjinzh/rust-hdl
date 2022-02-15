use rust_hdl::bsp::ok_xem6010::XEM6010;
use rust_hdl::core::prelude::*;
#[cfg(feature = "frontpanel")]
use test_common::mux_spi::*;

mod test_common;

#[cfg(feature = "frontpanel")]
#[test]
fn test_opalkelly_xem_6010_mux_spi() {
    let mut uut = OpalKellySPIMuxTest::new::<XEM6010>();
    uut.hi.link_connect_dest();
    uut.connect_all();
    rust_hdl::bsp::ok_xem6010::synth::synth_obj(uut, target_path!("xem_6010/mux_spi"));
    test_opalkelly_mux_spi_runtime(target_path!("xem_6010/mux_spi/top.bit")).unwrap()
}
