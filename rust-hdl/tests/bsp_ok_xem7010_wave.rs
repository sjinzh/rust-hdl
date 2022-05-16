use rust_hdl::bsp::ok_core::prelude::*;
#[cfg(feature = "frontpanel")]
use rust_hdl::bsp::ok_xem7010::*;
use rust_hdl::core::prelude::*;

mod test_common;
#[cfg(feature = "frontpanel")]
use test_common::wave::*;

#[cfg(feature = "frontpanel")]
#[test]
fn test_opalkelly_xem_7010_synth_wave() {
    let mut uut = OpalKellyWave::new::<XEM7010>();
    uut.hi.sig_in.connect();
    uut.hi.sig_out.connect();
    uut.hi.sig_inout.connect();
    uut.hi.sig_aa.connect();
    uut.connect_all();
    XEM7010::synth(uut, target_path!("xem_7010/wave"));
}
