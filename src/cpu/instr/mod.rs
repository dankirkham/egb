mod alu16;
mod alu8;
mod bit;
mod jump;
mod load16;
mod load8;
mod misc;
mod rot_shift;
mod undef;

use alu16::*;
use alu8::*;
use bit::*;
use jump::*;
use load16::*;
use load8::*;
use misc::*;
use rot_shift::*;
use undef::*;

use crate::cpu::{Cpu, Cycles};
use crate::memory::CpuMemory;

pub fn execute_instr(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    match mem.get_u8(cpu.pc) {
        0x00 => op00(cpu, mem),
        0x01 => op01(cpu, mem),
        0x02 => op02(cpu, mem),
        0x03 => op03(cpu, mem),
        0x04 => op04(cpu, mem),
        0x05 => op05(cpu, mem),
        0x06 => op06(cpu, mem),
        0x07 => op07(cpu, mem),
        0x08 => op08(cpu, mem),
        0x09 => op09(cpu, mem),
        0x0a => op0a(cpu, mem),
        0x0b => op0b(cpu, mem),
        0x0c => op0c(cpu, mem),
        0x0d => op0d(cpu, mem),
        0x0e => op0e(cpu, mem),
        0x0f => op0f(cpu, mem),
        0x10 => op10(cpu, mem),
        0x11 => op11(cpu, mem),
        0x12 => op12(cpu, mem),
        0x13 => op13(cpu, mem),
        0x14 => op14(cpu, mem),
        0x15 => op15(cpu, mem),
        0x16 => op16(cpu, mem),
        0x17 => op17(cpu, mem),
        0x18 => op18(cpu, mem),
        0x19 => op19(cpu, mem),
        0x1a => op1a(cpu, mem),
        0x1b => op1b(cpu, mem),
        0x1c => op1c(cpu, mem),
        0x1d => op1d(cpu, mem),
        0x1e => op1e(cpu, mem),
        0x1f => op1f(cpu, mem),
        0x20 => op20(cpu, mem),
        0x21 => op21(cpu, mem),
        0x22 => op22(cpu, mem),
        0x23 => op23(cpu, mem),
        0x24 => op24(cpu, mem),
        0x25 => op25(cpu, mem),
        0x26 => op26(cpu, mem),
        0x27 => op27(cpu, mem),
        0x28 => op28(cpu, mem),
        0x29 => op29(cpu, mem),
        0x2a => op2a(cpu, mem),
        0x2b => op2b(cpu, mem),
        0x2c => op2c(cpu, mem),
        0x2d => op2d(cpu, mem),
        0x2e => op2e(cpu, mem),
        0x2f => op2f(cpu, mem),
        0x30 => op30(cpu, mem),
        0x31 => op31(cpu, mem),
        0x32 => op32(cpu, mem),
        0x33 => op33(cpu, mem),
        0x34 => op34(cpu, mem),
        0x35 => op35(cpu, mem),
        0x36 => op36(cpu, mem),
        0x37 => op37(cpu, mem),
        0x38 => op38(cpu, mem),
        0x39 => op39(cpu, mem),
        0x3a => op3a(cpu, mem),
        0x3b => op3b(cpu, mem),
        0x3c => op3c(cpu, mem),
        0x3d => op3d(cpu, mem),
        0x3e => op3e(cpu, mem),
        0x3f => op3f(cpu, mem),
        0x40 => op40(cpu, mem),
        0x41 => op41(cpu, mem),
        0x42 => op42(cpu, mem),
        0x43 => op43(cpu, mem),
        0x44 => op44(cpu, mem),
        0x45 => op45(cpu, mem),
        0x46 => op46(cpu, mem),
        0x47 => op47(cpu, mem),
        0x48 => op48(cpu, mem),
        0x49 => op49(cpu, mem),
        0x4a => op4a(cpu, mem),
        0x4b => op4b(cpu, mem),
        0x4c => op4c(cpu, mem),
        0x4d => op4d(cpu, mem),
        0x4e => op4e(cpu, mem),
        0x4f => op4f(cpu, mem),
        0x50 => op50(cpu, mem),
        0x51 => op51(cpu, mem),
        0x52 => op52(cpu, mem),
        0x53 => op53(cpu, mem),
        0x54 => op54(cpu, mem),
        0x55 => op55(cpu, mem),
        0x56 => op56(cpu, mem),
        0x57 => op57(cpu, mem),
        0x58 => op58(cpu, mem),
        0x59 => op59(cpu, mem),
        0x5a => op5a(cpu, mem),
        0x5b => op5b(cpu, mem),
        0x5c => op5c(cpu, mem),
        0x5d => op5d(cpu, mem),
        0x5e => op5e(cpu, mem),
        0x5f => op5f(cpu, mem),
        0x60 => op60(cpu, mem),
        0x61 => op61(cpu, mem),
        0x62 => op62(cpu, mem),
        0x63 => op63(cpu, mem),
        0x64 => op64(cpu, mem),
        0x65 => op65(cpu, mem),
        0x66 => op66(cpu, mem),
        0x67 => op67(cpu, mem),
        0x68 => op68(cpu, mem),
        0x69 => op69(cpu, mem),
        0x6a => op6a(cpu, mem),
        0x6b => op6b(cpu, mem),
        0x6c => op6c(cpu, mem),
        0x6d => op6d(cpu, mem),
        0x6e => op6e(cpu, mem),
        0x6f => op6f(cpu, mem),
        0x70 => op70(cpu, mem),
        0x71 => op71(cpu, mem),
        0x72 => op72(cpu, mem),
        0x73 => op73(cpu, mem),
        0x74 => op74(cpu, mem),
        0x75 => op75(cpu, mem),
        0x76 => op76(cpu, mem),
        0x77 => op77(cpu, mem),
        0x78 => op78(cpu, mem),
        0x79 => op79(cpu, mem),
        0x7a => op7a(cpu, mem),
        0x7b => op7b(cpu, mem),
        0x7c => op7c(cpu, mem),
        0x7d => op7d(cpu, mem),
        0x7e => op7e(cpu, mem),
        0x7f => op7f(cpu, mem),
        0x80 => op80(cpu, mem),
        0x81 => op81(cpu, mem),
        0x82 => op82(cpu, mem),
        0x83 => op83(cpu, mem),
        0x84 => op84(cpu, mem),
        0x85 => op85(cpu, mem),
        0x86 => op86(cpu, mem),
        0x87 => op87(cpu, mem),
        0x88 => op88(cpu, mem),
        0x89 => op89(cpu, mem),
        0x8a => op8a(cpu, mem),
        0x8b => op8b(cpu, mem),
        0x8c => op8c(cpu, mem),
        0x8d => op8d(cpu, mem),
        0x8e => op8e(cpu, mem),
        0x8f => op8f(cpu, mem),
        0x90 => op90(cpu, mem),
        0x91 => op91(cpu, mem),
        0x92 => op92(cpu, mem),
        0x93 => op93(cpu, mem),
        0x94 => op94(cpu, mem),
        0x95 => op95(cpu, mem),
        0x96 => op96(cpu, mem),
        0x97 => op97(cpu, mem),
        0x98 => op98(cpu, mem),
        0x99 => op99(cpu, mem),
        0x9a => op9a(cpu, mem),
        0x9b => op9b(cpu, mem),
        0x9c => op9c(cpu, mem),
        0x9d => op9d(cpu, mem),
        0x9e => op9e(cpu, mem),
        0x9f => op9f(cpu, mem),
        0xa0 => opa0(cpu, mem),
        0xa1 => opa1(cpu, mem),
        0xa2 => opa2(cpu, mem),
        0xa3 => opa3(cpu, mem),
        0xa4 => opa4(cpu, mem),
        0xa5 => opa5(cpu, mem),
        0xa6 => opa6(cpu, mem),
        0xa7 => opa7(cpu, mem),
        0xa8 => opa8(cpu, mem),
        0xa9 => opa9(cpu, mem),
        0xaa => opaa(cpu, mem),
        0xab => opab(cpu, mem),
        0xac => opac(cpu, mem),
        0xad => opad(cpu, mem),
        0xae => opae(cpu, mem),
        0xaf => opaf(cpu, mem),
        0xb0 => opb0(cpu, mem),
        0xb1 => opb1(cpu, mem),
        0xb2 => opb2(cpu, mem),
        0xb3 => opb3(cpu, mem),
        0xb4 => opb4(cpu, mem),
        0xb5 => opb5(cpu, mem),
        0xb6 => opb6(cpu, mem),
        0xb7 => opb7(cpu, mem),
        0xb8 => opb8(cpu, mem),
        0xb9 => opb9(cpu, mem),
        0xba => opba(cpu, mem),
        0xbb => opbb(cpu, mem),
        0xbc => opbc(cpu, mem),
        0xbd => opbd(cpu, mem),
        0xbe => opbe(cpu, mem),
        0xbf => opbf(cpu, mem),
        0xc0 => opc0(cpu, mem),
        0xc1 => opc1(cpu, mem),
        0xc2 => opc2(cpu, mem),
        0xc3 => opc3(cpu, mem),
        0xc4 => opc4(cpu, mem),
        0xc5 => opc5(cpu, mem),
        0xc6 => opc6(cpu, mem),
        0xc7 => opc7(cpu, mem),
        0xc8 => opc8(cpu, mem),
        0xc9 => opc9(cpu, mem),
        0xca => opca(cpu, mem),
        0xcc => opcc(cpu, mem),
        0xcd => opcd(cpu, mem),
        0xce => opce(cpu, mem),
        0xcf => opcf(cpu, mem),
        0xd0 => opd0(cpu, mem),
        0xd1 => opd1(cpu, mem),
        0xd2 => opd2(cpu, mem),
        0xd4 => opd4(cpu, mem),
        0xd5 => opd5(cpu, mem),
        0xd6 => opd6(cpu, mem),
        0xd7 => opd7(cpu, mem),
        0xd8 => opd8(cpu, mem),
        0xd9 => opd9(cpu, mem),
        0xda => opda(cpu, mem),
        0xdc => opdc(cpu, mem),
        0xde => opde(cpu, mem),
        0xdf => opdf(cpu, mem),
        0xe0 => ope0(cpu, mem),
        0xe1 => ope1(cpu, mem),
        0xe2 => ope2(cpu, mem),
        0xe5 => ope5(cpu, mem),
        0xe6 => ope6(cpu, mem),
        0xe7 => ope7(cpu, mem),
        0xe8 => ope8(cpu, mem),
        0xe9 => ope9(cpu, mem),
        0xea => opea(cpu, mem),
        0xee => opee(cpu, mem),
        0xef => opef(cpu, mem),
        0xf0 => opf0(cpu, mem),
        0xf1 => opf1(cpu, mem),
        0xf2 => opf2(cpu, mem),
        0xf3 => opf3(cpu, mem),
        0xf5 => opf5(cpu, mem),
        0xf6 => opf6(cpu, mem),
        0xf7 => opf7(cpu, mem),
        0xf8 => opf8(cpu, mem),
        0xf9 => opf9(cpu, mem),
        0xfa => opfa(cpu, mem),
        0xfb => opfb(cpu, mem),
        0xfe => opfe(cpu, mem),
        0xff => opff(cpu, mem),

        0xd3 => opd3(cpu, mem),
        0xdb => opdb(cpu, mem),
        0xdd => opdd(cpu, mem),
        0xe3 => ope3(cpu, mem),
        0xe4 => ope4(cpu, mem),
        0xeb => opeb(cpu, mem),
        0xec => opec(cpu, mem),
        0xed => oped(cpu, mem),
        0xf4 => opf4(cpu, mem),
        0xfc => opfc(cpu, mem),
        0xfd => opfd(cpu, mem),

        0xcb => match mem.get_u8(cpu.pc.wrapping_add(1)) {
            0x00 => opcb00(cpu, mem),
            0x01 => opcb01(cpu, mem),
            0x02 => opcb02(cpu, mem),
            0x03 => opcb03(cpu, mem),
            0x04 => opcb04(cpu, mem),
            0x05 => opcb05(cpu, mem),
            0x06 => opcb06(cpu, mem),
            0x07 => opcb07(cpu, mem),
            0x08 => opcb08(cpu, mem),
            0x09 => opcb09(cpu, mem),
            0x0a => opcb0a(cpu, mem),
            0x0b => opcb0b(cpu, mem),
            0x0c => opcb0c(cpu, mem),
            0x0d => opcb0d(cpu, mem),
            0x0e => opcb0e(cpu, mem),
            0x0f => opcb0f(cpu, mem),
            0x10 => opcb10(cpu, mem),
            0x11 => opcb11(cpu, mem),
            0x12 => opcb12(cpu, mem),
            0x13 => opcb13(cpu, mem),
            0x14 => opcb14(cpu, mem),
            0x15 => opcb15(cpu, mem),
            0x16 => opcb16(cpu, mem),
            0x17 => opcb17(cpu, mem),
            0x18 => opcb18(cpu, mem),
            0x19 => opcb19(cpu, mem),
            0x1a => opcb1a(cpu, mem),
            0x1b => opcb1b(cpu, mem),
            0x1c => opcb1c(cpu, mem),
            0x1d => opcb1d(cpu, mem),
            0x1e => opcb1e(cpu, mem),
            0x1f => opcb1f(cpu, mem),
            0x20 => opcb20(cpu, mem),
            0x21 => opcb21(cpu, mem),
            0x22 => opcb22(cpu, mem),
            0x23 => opcb23(cpu, mem),
            0x24 => opcb24(cpu, mem),
            0x25 => opcb25(cpu, mem),
            0x26 => opcb26(cpu, mem),
            0x27 => opcb27(cpu, mem),
            0x28 => opcb28(cpu, mem),
            0x29 => opcb29(cpu, mem),
            0x2a => opcb2a(cpu, mem),
            0x2b => opcb2b(cpu, mem),
            0x2c => opcb2c(cpu, mem),
            0x2d => opcb2d(cpu, mem),
            0x2e => opcb2e(cpu, mem),
            0x2f => opcb2f(cpu, mem),
            0x30 => opcb30(cpu, mem),
            0x31 => opcb31(cpu, mem),
            0x32 => opcb32(cpu, mem),
            0x33 => opcb33(cpu, mem),
            0x34 => opcb34(cpu, mem),
            0x35 => opcb35(cpu, mem),
            0x36 => opcb36(cpu, mem),
            0x37 => opcb37(cpu, mem),
            0x38 => opcb38(cpu, mem),
            0x39 => opcb39(cpu, mem),
            0x3a => opcb3a(cpu, mem),
            0x3b => opcb3b(cpu, mem),
            0x3c => opcb3c(cpu, mem),
            0x3d => opcb3d(cpu, mem),
            0x3e => opcb3e(cpu, mem),
            0x3f => opcb3f(cpu, mem),
            0x40 => opcb40(cpu, mem),
            0x41 => opcb41(cpu, mem),
            0x42 => opcb42(cpu, mem),
            0x43 => opcb43(cpu, mem),
            0x44 => opcb44(cpu, mem),
            0x45 => opcb45(cpu, mem),
            0x46 => opcb46(cpu, mem),
            0x47 => opcb47(cpu, mem),
            0x48 => opcb48(cpu, mem),
            0x49 => opcb49(cpu, mem),
            0x4a => opcb4a(cpu, mem),
            0x4b => opcb4b(cpu, mem),
            0x4c => opcb4c(cpu, mem),
            0x4d => opcb4d(cpu, mem),
            0x4e => opcb4e(cpu, mem),
            0x4f => opcb4f(cpu, mem),
            0x50 => opcb50(cpu, mem),
            0x51 => opcb51(cpu, mem),
            0x52 => opcb52(cpu, mem),
            0x53 => opcb53(cpu, mem),
            0x54 => opcb54(cpu, mem),
            0x55 => opcb55(cpu, mem),
            0x56 => opcb56(cpu, mem),
            0x57 => opcb57(cpu, mem),
            0x58 => opcb58(cpu, mem),
            0x59 => opcb59(cpu, mem),
            0x5a => opcb5a(cpu, mem),
            0x5b => opcb5b(cpu, mem),
            0x5c => opcb5c(cpu, mem),
            0x5d => opcb5d(cpu, mem),
            0x5e => opcb5e(cpu, mem),
            0x5f => opcb5f(cpu, mem),
            0x60 => opcb60(cpu, mem),
            0x61 => opcb61(cpu, mem),
            0x62 => opcb62(cpu, mem),
            0x63 => opcb63(cpu, mem),
            0x64 => opcb64(cpu, mem),
            0x65 => opcb65(cpu, mem),
            0x66 => opcb66(cpu, mem),
            0x67 => opcb67(cpu, mem),
            0x68 => opcb68(cpu, mem),
            0x69 => opcb69(cpu, mem),
            0x6a => opcb6a(cpu, mem),
            0x6b => opcb6b(cpu, mem),
            0x6c => opcb6c(cpu, mem),
            0x6d => opcb6d(cpu, mem),
            0x6e => opcb6e(cpu, mem),
            0x6f => opcb6f(cpu, mem),
            0x70 => opcb70(cpu, mem),
            0x71 => opcb71(cpu, mem),
            0x72 => opcb72(cpu, mem),
            0x73 => opcb73(cpu, mem),
            0x74 => opcb74(cpu, mem),
            0x75 => opcb75(cpu, mem),
            0x76 => opcb76(cpu, mem),
            0x77 => opcb77(cpu, mem),
            0x78 => opcb78(cpu, mem),
            0x79 => opcb79(cpu, mem),
            0x7a => opcb7a(cpu, mem),
            0x7b => opcb7b(cpu, mem),
            0x7c => opcb7c(cpu, mem),
            0x7d => opcb7d(cpu, mem),
            0x7e => opcb7e(cpu, mem),
            0x7f => opcb7f(cpu, mem),
            0x80 => opcb80(cpu, mem),
            0x81 => opcb81(cpu, mem),
            0x82 => opcb82(cpu, mem),
            0x83 => opcb83(cpu, mem),
            0x84 => opcb84(cpu, mem),
            0x85 => opcb85(cpu, mem),
            0x86 => opcb86(cpu, mem),
            0x87 => opcb87(cpu, mem),
            0x88 => opcb88(cpu, mem),
            0x89 => opcb89(cpu, mem),
            0x8a => opcb8a(cpu, mem),
            0x8b => opcb8b(cpu, mem),
            0x8c => opcb8c(cpu, mem),
            0x8d => opcb8d(cpu, mem),
            0x8e => opcb8e(cpu, mem),
            0x8f => opcb8f(cpu, mem),
            0x90 => opcb90(cpu, mem),
            0x91 => opcb91(cpu, mem),
            0x92 => opcb92(cpu, mem),
            0x93 => opcb93(cpu, mem),
            0x94 => opcb94(cpu, mem),
            0x95 => opcb95(cpu, mem),
            0x96 => opcb96(cpu, mem),
            0x97 => opcb97(cpu, mem),
            0x98 => opcb98(cpu, mem),
            0x99 => opcb99(cpu, mem),
            0x9a => opcb9a(cpu, mem),
            0x9b => opcb9b(cpu, mem),
            0x9c => opcb9c(cpu, mem),
            0x9d => opcb9d(cpu, mem),
            0x9e => opcb9e(cpu, mem),
            0x9f => opcb9f(cpu, mem),
            0xa0 => opcba0(cpu, mem),
            0xa1 => opcba1(cpu, mem),
            0xa2 => opcba2(cpu, mem),
            0xa3 => opcba3(cpu, mem),
            0xa4 => opcba4(cpu, mem),
            0xa5 => opcba5(cpu, mem),
            0xa6 => opcba6(cpu, mem),
            0xa7 => opcba7(cpu, mem),
            0xa8 => opcba8(cpu, mem),
            0xa9 => opcba9(cpu, mem),
            0xaa => opcbaa(cpu, mem),
            0xab => opcbab(cpu, mem),
            0xac => opcbac(cpu, mem),
            0xad => opcbad(cpu, mem),
            0xae => opcbae(cpu, mem),
            0xaf => opcbaf(cpu, mem),
            0xb0 => opcbb0(cpu, mem),
            0xb1 => opcbb1(cpu, mem),
            0xb2 => opcbb2(cpu, mem),
            0xb3 => opcbb3(cpu, mem),
            0xb4 => opcbb4(cpu, mem),
            0xb5 => opcbb5(cpu, mem),
            0xb6 => opcbb6(cpu, mem),
            0xb7 => opcbb7(cpu, mem),
            0xb8 => opcbb8(cpu, mem),
            0xb9 => opcbb9(cpu, mem),
            0xba => opcbba(cpu, mem),
            0xbb => opcbbb(cpu, mem),
            0xbc => opcbbc(cpu, mem),
            0xbd => opcbbd(cpu, mem),
            0xbe => opcbbe(cpu, mem),
            0xbf => opcbbf(cpu, mem),
            0xc0 => opcbc0(cpu, mem),
            0xc1 => opcbc1(cpu, mem),
            0xc2 => opcbc2(cpu, mem),
            0xc3 => opcbc3(cpu, mem),
            0xc4 => opcbc4(cpu, mem),
            0xc5 => opcbc5(cpu, mem),
            0xc6 => opcbc6(cpu, mem),
            0xc7 => opcbc7(cpu, mem),
            0xc8 => opcbc8(cpu, mem),
            0xc9 => opcbc9(cpu, mem),
            0xca => opcbca(cpu, mem),
            0xcb => opcbcb(cpu, mem),
            0xcc => opcbcc(cpu, mem),
            0xcd => opcbcd(cpu, mem),
            0xce => opcbce(cpu, mem),
            0xcf => opcbcf(cpu, mem),
            0xd0 => opcbd0(cpu, mem),
            0xd1 => opcbd1(cpu, mem),
            0xd2 => opcbd2(cpu, mem),
            0xd3 => opcbd3(cpu, mem),
            0xd4 => opcbd4(cpu, mem),
            0xd5 => opcbd5(cpu, mem),
            0xd6 => opcbd6(cpu, mem),
            0xd7 => opcbd7(cpu, mem),
            0xd8 => opcbd8(cpu, mem),
            0xd9 => opcbd9(cpu, mem),
            0xda => opcbda(cpu, mem),
            0xdb => opcbdb(cpu, mem),
            0xdc => opcbdc(cpu, mem),
            0xdd => opcbdd(cpu, mem),
            0xde => opcbde(cpu, mem),
            0xdf => opcbdf(cpu, mem),
            0xe0 => opcbe0(cpu, mem),
            0xe1 => opcbe1(cpu, mem),
            0xe2 => opcbe2(cpu, mem),
            0xe3 => opcbe3(cpu, mem),
            0xe4 => opcbe4(cpu, mem),
            0xe5 => opcbe5(cpu, mem),
            0xe6 => opcbe6(cpu, mem),
            0xe7 => opcbe7(cpu, mem),
            0xe8 => opcbe8(cpu, mem),
            0xe9 => opcbe9(cpu, mem),
            0xea => opcbea(cpu, mem),
            0xeb => opcbeb(cpu, mem),
            0xec => opcbec(cpu, mem),
            0xed => opcbed(cpu, mem),
            0xee => opcbee(cpu, mem),
            0xef => opcbef(cpu, mem),
            0xf0 => opcbf0(cpu, mem),
            0xf1 => opcbf1(cpu, mem),
            0xf2 => opcbf2(cpu, mem),
            0xf3 => opcbf3(cpu, mem),
            0xf4 => opcbf4(cpu, mem),
            0xf5 => opcbf5(cpu, mem),
            0xf6 => opcbf6(cpu, mem),
            0xf7 => opcbf7(cpu, mem),
            0xf8 => opcbf8(cpu, mem),
            0xf9 => opcbf9(cpu, mem),
            0xfa => opcbfa(cpu, mem),
            0xfb => opcbfb(cpu, mem),
            0xfc => opcbfc(cpu, mem),
            0xfd => opcbfd(cpu, mem),
            0xfe => opcbfe(cpu, mem),
            0xff => opcbff(cpu, mem),
        },
    }
}
