use crate::resistors::{PowerWatt, ResistorKind};
use crate::smd::SizeCode;
use crate::yageo_resistor_series::make_yageo_series_resistor;
use crate::tdk_cga_series::make_tdk_cga_capacitor;
use crate::capacitors::{CapacitorKind, DielectricCode, CapacitorTolerance};
use crate::kemet_t491_series::make_kemet_t491_capacitor;
use crate::avx_caps::make_avx_capacitor;
use crate::kemet_ceramic_caps::make_kemet_ceramic_capacitor;
use crate::tdk_c_series::make_tdk_c_series_capacitor;
use crate::yageo_cc_caps::make_yageo_cc_series_cap;
use crate::murata_mlcc_caps::{make_murata_capacitor};
use crate::panasonic_era_resistors::{make_panasonic_resistor};
use crate::nippon_electrolytic_caps::make_nippon_hxd_capacitor;
use crate::wurth_led::make_wurth_led;
use crate::diode::DiodeKind;
use crate::epin::{PinKind, PassiveKind, SignalKind};
use crate::ldo::{make_zldo1117g_regulator, make_ti_tps_7b84_regulator, make_mcp_1799_regulator, make_on_semi_ncv33375_regulator};
use crate::analog_devices::make_lt3092_current_source;

mod bom;
mod capacitors;
mod circuit;
mod designator;
mod digikey_table;
mod epin;
mod inductors;
mod murata_mlcc_caps;
mod resistors;
mod smd;
mod tdk_cga_series;
mod traco_power_tmr1_series;
mod yageo_resistor_series;
mod kemet_t491_series;
mod avx_caps;
mod kemet_ceramic_caps;
mod tdk_c_series;
mod yageo_cc_caps;
mod panasonic_era_resistors;
mod utils;
mod nippon_electrolytic_caps;
mod wurth_led;
mod diode;
mod ldo;
mod analog_devices;

#[test]
fn test_yageo_rc_68k() {
    let led_limit = make_yageo_series_resistor("RC0603FR-0768KL");
    println!("{:?}", led_limit);
    assert_eq!(led_limit.value_ohms, 68e3);
    assert_eq!(led_limit.details.size, SizeCode::I0603);
    assert!(led_limit.power_watt >= PowerWatt::new(1, 10));
}

#[test]
fn test_yageo_rc_1k() {
    let current_limit = make_yageo_series_resistor("RC1206FR-071KL");
    assert_eq!(current_limit.value_ohms, 1.0e3);
    assert_eq!(current_limit.details.size, SizeCode::I1206);
    assert!(current_limit.power_watt >= PowerWatt::new(1, 4));
    assert_eq!(current_limit.details.pins.len(), 2);
}

#[test]
fn test_tdk_cga_cap() {
    let filter_cap = make_tdk_cga_capacitor("CGA4J2X7R2A104K125AA");
    // 'TDK         CGA4J2X7R2A104K125AA             SMD Multilayer Ceramic Capacitor, 0805 [2012 Metric], 0.1 F, 100 V,  10%, X7R, CGA Series
    assert_eq!(filter_cap.details.manufacturer.name, "TDK");
    assert_eq!(filter_cap.kind, CapacitorKind::MultiLayerChip(DielectricCode::X7R));
    assert_eq!(filter_cap.details.size, SizeCode::I0805);
    assert_eq!(filter_cap.value_pf, 0.1 * 1e6);
    assert_eq!(filter_cap.voltage, 100.);
    println!("{:?}", filter_cap);
}

#[test]
fn test_kemet_tantalum_cap() {
    let kemet = make_kemet_t491_capacitor("T491A106K010AT");
    assert_eq!(kemet.details.size, SizeCode::I1206);
    assert_eq!(kemet.kind, CapacitorKind::Tantalum);
    assert_eq!(kemet.value_pf, 10. * 1e6);
    assert_eq!(kemet.voltage, 10.);
    assert_eq!(kemet.tolerance, CapacitorTolerance::TenPercent);
    println!("{:#?}", kemet);
}

#[test]
fn test_yageo_precision() {
    let precise = make_yageo_series_resistor("RL0603FR-070R47L");
    // 'Res Thick Film 0603 0.47 Ohm 1% 0.1W(1/10W) ±800ppm/C Molded SMD Paper T/R
    assert_eq!(precise.details.size, SizeCode::I0603);
    assert_eq!(precise.tolerance, 1.0);
    assert_eq!(precise.value_ohms, 0.47);
}

#[test]
fn test_yageo_bulk() {
    let bulk = make_yageo_series_resistor("RC1206FR-071KL");
    // 'YAGEO - RC1206FR-071KL. - RES, THICK FILM, 1K, 1%, 0.25W, 1206, REEL
    assert_eq!(bulk.tolerance, 1.0);
    assert_eq!(bulk.power_watt, PowerWatt::new(1, 4));
    assert_eq!(bulk.details.size, SizeCode::I1206);
    assert_eq!(bulk.value_ohms, 1.0e3);
}

#[test]
fn test_yageo_tempco() {
    // T491A106K010AT
    let temp = make_yageo_series_resistor("AT0603BRD0720KL");
    // '20K 0.1% precision
    assert_eq!(temp.tolerance, 0.1);
    assert_eq!(temp.value_ohms, 20e3);
    assert_eq!(temp.tempco, Some(25.0));
}

#[test]
fn test_avx() {
    let avx = make_avx_capacitor("22201C475KAT2A");
    assert_eq!(avx.details.size, SizeCode::I2220);
    assert_eq!(avx.kind, CapacitorKind::MultiLayerChip(DielectricCode::X7R));
    assert_eq!(avx.value_pf, 47e5);
    assert_eq!(avx.tolerance, CapacitorTolerance::TenPercent);
}

#[test]
fn test_kemet() {
    let c = make_kemet_ceramic_capacitor("C0603C104K5RACTU");
    assert_eq!(c.details.size, SizeCode::I0603);
    assert_eq!(c.kind, CapacitorKind::MultiLayerChip(DielectricCode::X7R));
    assert_eq!(c.value_pf, 10e4);
    assert_eq!(c.tolerance, CapacitorTolerance::TenPercent);
}

#[test]
fn test_tdk_c_series() {
    let c = make_tdk_c_series_capacitor("C1608X7R1C105K080AC");
    assert_eq!(c.details.size, SizeCode::I0603);
    assert_eq!(c.kind, CapacitorKind::MultiLayerChip(DielectricCode::X7R));
    assert_eq!(c.value_pf, 10e5);
    assert_eq!(c.tolerance, CapacitorTolerance::TenPercent);
    assert_eq!(c.voltage, 16.0);
}

#[test]
fn test_yageo_cc_series() {
    let c = make_yageo_cc_series_cap("CC0805KKX5R8BB106");
    // 'Cap Ceramic 10uF 25V X5R 10% Pad SMD 0805 85°C T/R
    assert_eq!(c.details.size, SizeCode::I0805);
    assert_eq!(c.tolerance, CapacitorTolerance::TenPercent);
    assert_eq!(c.voltage, 25.0);
    assert_eq!(c.kind, CapacitorKind::MultiLayerChip(DielectricCode::X5R));
    assert_eq!(c.value_pf, 10.*1e6);
}

#[test]
fn test_murata_grt_series() {
    let c = make_murata_capacitor("GRT188R61H105KE13D");
    // 'Multilayer Ceramic Capacitors MLCC - SMD/SMT 0603 50Vdc 1.0uF X5R 10%
    assert_eq!(c.details.size, SizeCode::I0603);
    assert_eq!(c.tolerance, CapacitorTolerance::TenPercent);
    assert_eq!(c.voltage, 50.0);
    assert_eq!(c.kind, CapacitorKind::MultiLayerChip(DielectricCode::X5R));
    assert_eq!(c.value_pf, 10.*1e5);
}

#[test]
fn test_panasonic_era_series() {
    let r = make_panasonic_resistor("ERA8AEB201V");
    // 'RES SMD 200 OHM 0.1% 1/4W 1206
    assert_eq!(r.details.size, SizeCode::I1206);
    assert_eq!(r.tolerance, 0.1);
    assert_eq!(r.value_ohms, 200.);
    assert_eq!(r.power_watt, PowerWatt::new(1, 4));
    assert_eq!(r.kind, ResistorKind::ThinFilmChip);
    assert_eq!(r.tempco, Some(25.));
}

#[test]
fn test_panasonic_erj_series() {
    let r = make_panasonic_resistor("ERJ-3RQFR22V");
    // 'Res Thick Film 0603 0.22 Ohm 1% 1/10W ±300ppm/°C Molded SMD Punched Carrier T/R
    assert_eq!(r.details.size, SizeCode::I0603);
    assert_eq!(r.tolerance, 1.);
    assert_eq!(r.value_ohms, 0.22);
    assert_eq!(r.power_watt, PowerWatt::new(1, 10));
    assert_eq!(r.kind, ResistorKind::ThickFilmChip);
}

#[test]
fn test_murata_grm_series() {
    let c = make_murata_capacitor("GRM21BR61C226ME44L");
    // '0805 22 uF 16 V ±20% Tolerance X5R Multilayer Ceramic Chip Capacitor
    assert_eq!(c.details.size, SizeCode::I0805);
    assert_eq!(c.voltage, 16.);
    assert_eq!(c.tolerance, CapacitorTolerance::TwentyPercent);
    assert_eq!(c.kind, CapacitorKind::MultiLayerChip(DielectricCode::X5R));
    assert_eq!(c.value_pf, 22e6);
}

#[test]
fn test_chemi_con_hybrid_cap() {
    let c = make_nippon_hxd_capacitor("HHXD500ARA101MJA0G");
    // 100 uF, 50V Alum Poly 25 mR ESR, Hybrid
    assert_eq!(c.voltage, 50.);
    assert_eq!(c.kind, CapacitorKind::AluminumPolyLowESR(25));
    assert_eq!(c.value_pf, 100.*1e6);
    assert_eq!(c.details.size, SizeCode::Custom("JA0".to_owned()))
}

#[test]
fn test_yageo_pth_resistors() {
    let r = make_yageo_series_resistor("FMP100JR-52-15K");
    assert_eq!(r.tolerance, 5.);
    assert_eq!(r.power_watt, PowerWatt::new(1, 1));
    assert_eq!(r.value_ohms, 15e3);
    let r = make_yageo_series_resistor("FMP100JR-52-10R");
    assert_eq!(r.tolerance, 5.);
    assert_eq!(r.power_watt, PowerWatt::new(1, 1));
    assert_eq!(r.value_ohms, 10.0);
}

#[test]
fn test_green_led() {
    let d = make_wurth_led("150060GS75000");
    assert_eq!(d.kind, DiodeKind::LED("Green".into()));
    assert_eq!(d.forward_drop_volts, 3.2);
    assert_eq!(d.details.pins.len(), 2);
    assert_eq!(d.details.pins[&1].kind, PinKind::Passive(PassiveKind::Negative));
    assert_eq!(d.details.pins[&2].kind, PinKind::Passive(PassiveKind::Positive));
}

#[test]
fn test_zldo_regulator() {
    let v = make_zldo1117g_regulator("ZLDO1117G50TA");
    assert_eq!(v.input_max_voltage, 18.0);
    assert_eq!(v.output_nominal_voltage, 5.0);
    assert_eq!(v.details.pins.len(), 4);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&2].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerSource);
    let v = make_zldo1117g_regulator("ZLDO1117G33TA");
    assert_eq!(v.input_max_voltage, 18.0);
    assert_eq!(v.output_nominal_voltage, 3.3);
    assert_eq!(v.details.pins.len(), 4);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&2].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerSource);
}

#[test]
fn test_ti_tps_regulator() {
    let v = make_ti_tps_7b84_regulator("TPS7B8450QDCYRQ1");
    assert_eq!(v.input_max_voltage, 42.0);
    assert_eq!(v.output_nominal_voltage, 5.0);
    assert_eq!(v.output_max_current_ma, 150.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "IN");
    assert_eq!(v.details.pins[&2].kind, PinKind::Input(SignalKind::Any));
    assert_eq!(v.details.pins[&2].name, "EN");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "OUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND");
    let v = make_ti_tps_7b84_regulator("TPS7B8433QDCYRQ1");
    assert_eq!(v.input_max_voltage, 42.0);
    assert_eq!(v.output_nominal_voltage, 3.3);
    assert_eq!(v.output_max_current_ma, 150.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "IN");
    assert_eq!(v.details.pins[&2].kind, PinKind::Input(SignalKind::Any));
    assert_eq!(v.details.pins[&2].name, "EN");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "OUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND");
}

#[test]
fn test_on_semi_regulators() {
    let v = make_on_semi_ncv33375_regulator("NCV33375ST3.3T3G");
    assert_eq!(v.input_max_voltage, 13.0);
    assert_eq!(v.output_nominal_voltage, 3.3);
    assert_eq!(v.output_max_current_ma, 300.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "VIN");
    assert_eq!(v.details.pins[&2].kind, PinKind::Input(SignalKind::Any));
    assert_eq!(v.details.pins[&2].name, "ON/OFF");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "VOUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND");
    let v = make_on_semi_ncv33375_regulator("NCV33375ST1.8T3G");
    assert_eq!(v.input_max_voltage, 13.0);
    assert_eq!(v.output_nominal_voltage, 1.8);
    assert_eq!(v.output_max_current_ma, 300.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "VIN");
    assert_eq!(v.details.pins[&2].kind, PinKind::Input(SignalKind::Any));
    assert_eq!(v.details.pins[&2].name, "ON/OFF");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "VOUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND");
}

#[test]
fn test_microchip_regulators() {
    let v = make_mcp_1799_regulator("MCP1799T-5002H/DB");
    assert_eq!(v.input_max_voltage, 45.0);
    assert_eq!(v.output_nominal_voltage, 5.0);
    assert_eq!(v.output_max_current_ma, 80.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "VIN");
    assert_eq!(v.details.pins[&2].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&2].name, "GND_1");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "VOUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND_2");
    let v = make_mcp_1799_regulator("MCP1799T-3302H/DB");
    assert_eq!(v.input_max_voltage, 45.0);
    assert_eq!(v.output_nominal_voltage, 3.3);
    assert_eq!(v.output_max_current_ma, 80.0);
    assert_eq!(v.details.pins[&1].kind, PinKind::PowerSink);
    assert_eq!(v.details.pins[&1].name, "VIN");
    assert_eq!(v.details.pins[&2].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&2].name, "GND_1");
    assert_eq!(v.details.pins[&3].kind, PinKind::PowerSource);
    assert_eq!(v.details.pins[&3].name, "VOUT");
    assert_eq!(v.details.pins[&4].kind, PinKind::PowerReturn);
    assert_eq!(v.details.pins[&4].name, "GND_2");
}

#[test]
fn test_lt3092() {
    let u = make_lt3092_current_source("LT3092EST#PBF");
    assert_eq!(u.pins.len(), 4);
    assert_eq!(u.pins[&1].kind, PinKind::Input(SignalKind::Any));
    assert_eq!(u.pins[&1].name, "SET");
    assert_eq!(u.pins[&2].kind, PinKind::PowerSource);
    assert_eq!(u.pins[&2].name, "OUT_1");
    assert_eq!(u.pins[&3].kind, PinKind::PowerSink);
    assert_eq!(u.pins[&3].name, "IN");
    assert_eq!(u.pins[&4].kind, PinKind::PowerSource);
    assert_eq!(u.pins[&4].name, "OUT_2");
}