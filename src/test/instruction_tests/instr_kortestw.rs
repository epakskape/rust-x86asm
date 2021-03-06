use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kortestw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTW, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 152, 226], OperandSize::Dword)
}

#[test]
fn kortestw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTW, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 152, 230], OperandSize::Qword)
}

