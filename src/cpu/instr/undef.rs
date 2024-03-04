use crate::cpu::{Cpu, Cycles};
use crate::memory::ProgramMemory;

macro_rules! undef {
    ( $name:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            println!("{}", cpu);
            panic!("Undefined instruction used!");
        }
    };
}
undef!(opd3);
undef!(opdb);
undef!(opdd);
undef!(ope3);
undef!(ope4);
undef!(opeb);
undef!(opec);
undef!(oped);
undef!(opf4);
undef!(opfc);
undef!(opfd);
