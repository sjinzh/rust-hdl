use crate::core::ast::{Verilog, VerilogLiteral};
use crate::core::atom::AtomKind::{StubInputSignal, StubOutputSignal};
use crate::core::atom::{Atom, AtomKind, get_atom_typename, is_atom_an_enum, is_atom_signed};
use crate::core::block::Block;
use crate::core::code_writer::CodeWriter;
use crate::core::named_path::NamedPath;
use crate::core::probe::Probe;
use crate::core::verilog_gen::verilog_combinatorial;
use std::collections::BTreeMap;
use crate::core::check_error::check_all;
use crate::core::type_descriptor::{TypeDescriptor, TypeKind};
use crate::wait_clock_true;

#[derive(Clone, Debug, Default)]
struct SubModuleInvocation {
    kind: String,
    name: String,
}

#[derive(Clone, Debug, Default)]
struct ModuleDetails {
    atoms: Vec<AtomDetails>,
    sub_modules: Vec<SubModuleInvocation>,
    enums: Vec<EnumDefinition>,
    code: Verilog,
}

#[derive(Clone, Debug, PartialEq)]
struct EnumDefinition {
    pub type_name: String,
    pub discriminant: String,
    pub value: usize,
}

#[derive(Clone, Debug)]
struct AtomDetails {
    name: String,
    kind: AtomKind,
    width: usize,
    const_val: VerilogLiteral,
    signed: bool,
}

fn verilog_atom_name(x: &AtomKind) -> &str {
    match x {
        AtomKind::InputParameter => "input wire",
        AtomKind::OutputParameter => "output reg",
        AtomKind::StubInputSignal => "reg",
        AtomKind::StubOutputSignal => "wire",
        AtomKind::Constant => "localparam",
        AtomKind::LocalSignal => "reg",
        AtomKind::InOutParameter => "inout wire",
        AtomKind::OutputPassthrough => "output wire",
    }
}

fn decl(x: &AtomDetails) -> String {
    let signed = if x.signed { "signed" } else { "" };
    if x.kind == AtomKind::Constant {
        format!(
            "{} {} {} = {};",
            verilog_atom_name(&x.kind),
            signed,
            x.name,
            x.const_val
        )
    } else {
        if x.width == 1 {
            format!("{} {} {};", verilog_atom_name(&x.kind), signed, x.name)
        } else {
            format!(
                "{} {} [{}:0] {};",
                verilog_atom_name(&x.kind),
                signed,
                x.width - 1,
                x.name
            )
        }
    }
}

#[derive(Default)]
pub struct ModuleDefines {
    path: NamedPath,
    namespace: NamedPath,
    details: BTreeMap<String, ModuleDetails>,
}

impl ModuleDefines {
    fn add_atom(&mut self, module: &str, atom: AtomDetails) {
        let entry = self.details.entry(module.into()).or_default();
        entry.atoms.push(atom)
    }
    fn add_submodule(&mut self, module: &str, name: &str, kind: &str) {
        let entry = self.details.entry(module.into()).or_default();
        entry.sub_modules.push(SubModuleInvocation {
            kind: kind.to_owned(),
            name: name.to_owned(),
        });
    }
    fn add_enums(&mut self, module: &str, descriptor: &TypeDescriptor) {
        let entry = self.details.entry(module.into()).or_default();
        let enum_name = descriptor.name.clone();
        match &descriptor.kind {
            TypeKind::Enum(x) => {
                for (ndx, label) in x.iter().enumerate() {
                    let def = EnumDefinition {
                        type_name: enum_name.clone(),
                        discriminant: label.into(),
                        value: ndx
                    };
                    if !entry.enums.contains(&def) {
                        entry.enums.push(def);
                    }
                }
            }
            TypeKind::Composite(x) => {
                for item in x {
                    self.add_enums(module, &item.kind);
                }
            }
            _ => {}
        }
    }
    fn add_code(&mut self, module: &str, code: Verilog) {
        let entry = self.details.entry(module.into()).or_default();
        entry.code = code;
    }
}

impl Probe for ModuleDefines {
    fn visit_start_scope(&mut self, name: &str, node: &dyn Block) {
        let top_level = self.path.to_string();
        self.path.push(name);
        self.namespace.reset();
        self.add_submodule(&top_level, name, &self.path.to_string());
        self.add_code(&self.path.to_string(), node.hdl());
    }

    fn visit_start_namespace(&mut self, name: &str, _node: &dyn Block) {
        self.namespace.push(name);
    }

    fn visit_atom(&mut self, name: &str, signal: &dyn Atom) {
        let module_path = self.path.to_string();
        let module_name = self.path.last();
        let namespace = self.namespace.flat("$");
        let name = if namespace.is_empty() {
            name.to_owned()
        } else {
            format!("{}${}", namespace, name)
        };
        let param = AtomDetails {
            name: name.clone(),
            kind: signal.kind(),
            width: signal.bits(),
            const_val: signal.verilog(),
            signed: is_atom_signed(signal),
        };
        if param.kind.is_parameter() {
            let kind = if param.kind == AtomKind::InputParameter {
                StubInputSignal
            } else {
                StubOutputSignal
            };
            let parent_param = AtomDetails {
                name: format!("{}${}", module_name, name.to_owned()),
                kind,
                width: signal.bits(),
                const_val: signal.verilog(),
                signed: is_atom_signed(signal),
            };
            let parent_name = self.path.parent();
            self.add_atom(&parent_name, parent_param);
        }
        self.add_enums(&module_path, &signal.descriptor());
        self.add_enums(&self.path.parent(), &signal.descriptor());
        self.add_atom(&module_path, param);
    }

    fn visit_end_namespace(&mut self, _name: &str, _node: &dyn Block) {
        self.namespace.pop();
    }

    fn visit_end_scope(&mut self, _name: &str, _node: &dyn Block) {
        self.path.pop();
    }
}

impl ModuleDefines {
    pub fn defines(&self) -> String {
        let mut io = CodeWriter::new();
        self.details
            .iter()
            .filter(|x| x.0.len() != 0)
            .filter(|x| !matches!(x.1.code, Verilog::Blackbox(_)))
            .for_each(|k| {
                let module_name = k.0;
                let module_details = k.1;
                // Remap the output parameters to pass throughs (net type) in case we have a wrapper
                let atoms_passthrough = &module_details.atoms.iter().map(|x| {
                    let mut y = x.clone();
                    if y.kind == AtomKind::OutputParameter {
                        y.kind = AtomKind::OutputPassthrough;
                    }
                    y
                }).collect::<Vec<_>>();
                let wrapper_mode = if let Verilog::Wrapper(_) = &module_details.code {
                    true
                } else {
                    false
                };
                let atoms = if wrapper_mode {
                    io.add("\n// v-- Setting output parameters to net type for wrapped code.\n");
                    &atoms_passthrough
                } else {
                    &module_details.atoms
                };
                let args = atoms
                    .iter()
                    .filter(|x| x.kind.is_parameter())
                    .collect::<Vec<_>>();
                let stubs = atoms
                    .iter()
                    .filter(|x| x.kind.is_stub())
                    .collect::<Vec<_>>();
                let consts = atoms
                    .iter()
                    .filter(|x| x.kind == AtomKind::Constant)
                    .collect::<Vec<_>>();
                let locals = atoms
                    .iter()
                    .filter(|x| x.kind == AtomKind::LocalSignal)
                    .collect::<Vec<_>>();
                let module_args = args
                    .iter()
                    .map(|x| x.name.to_owned())
                    .collect::<Vec<_>>()
                    .join(",");
                io.add(format!("\n\nmodule {}({});", module_name, module_args));
                io.push();
                if !args.is_empty() {
                    io.add("\n// Module arguments");
                    args.iter().for_each(|x| io.add(decl(x)));
                }
                let submodules = &module_details.sub_modules;
                if !consts.is_empty() {
                    io.add("\n// Constant declarations");
                    consts.iter().for_each(|x| io.add(decl(x)));
                }
                if !module_details.enums.is_empty() & !wrapper_mode {
                    io.add("\n// Enums");
                    module_details.enums.iter().for_each(|x| {
                        io.add(format!(
                            "localparam {} = {};",
                            x.discriminant.replace("::", "$"),
                            x.value
                        ))
                    });
                }
                if !stubs.is_empty() & !wrapper_mode {
                    io.add("\n// Stub signals");
                    stubs.iter().for_each(|x| io.add(decl(x)));
                }
                if !locals.is_empty() & !wrapper_mode {
                    io.add("\n// Local signals");
                    locals.iter().for_each(|x| io.add(decl(x)));
                }
                if !submodules.is_empty() & !wrapper_mode {
                    io.add("\n// Sub module instances");
                    for child in submodules {
                        let entry = self.details.get(&child.kind).unwrap();
                        let submodule_kind = match &entry.code {
                            Verilog::Blackbox(b) => &b.name,
                            _ => &child.kind,
                        };
                        let child_args = entry
                            .atoms
                            .iter()
                            .filter(|x| x.kind.is_parameter())
                            .map(|x| format!(".{}({}${})", x.name, child.name, x.name))
                            .collect::<Vec<_>>()
                            .join(",");
                        io.add(format!(
                            "{} {}({});",
                            submodule_kind, child.name, child_args
                        ))
                    }
                }
                match &module_details.code {
                    Verilog::Combinatorial(code) => {
                        io.add("\n// Update code");
                        io.add(verilog_combinatorial(code));
                    }
                    Verilog::Custom(code) => {
                        io.add("\n// Update code (custom)");
                        io.add(code);
                    }
                    Verilog::Wrapper(c) => {
                        io.add("\n// Update code (wrapper)");
                        io.add(&c.code);
                    }
                    Verilog::Blackbox(_) => {}
                    Verilog::Empty => {}
                }
                io.pop();
                io.add(format!("endmodule // {}", module_name));
            });
        self.details.iter().for_each(|x| match &x.1.code {
            Verilog::Blackbox(b) => io.add(&b.code),
            Verilog::Wrapper(w) => io.add(&w.cores),
            _ => {}
        });
        io.to_string()
    }
}

pub fn generate_verilog<U: Block>(uut: &U) -> String {
    let mut defines = ModuleDefines::default();
    check_all(uut).unwrap(); // TODO - make this not panic...
    uut.accept("top", &mut defines);
    defines.defines()
}

pub fn generate_verilog_unchecked<U: Block>(uut: &U) -> String {
    let mut defines = ModuleDefines::default();
    uut.accept("top", &mut defines);
    defines.defines()
}
