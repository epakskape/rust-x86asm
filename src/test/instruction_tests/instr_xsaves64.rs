use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaves64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES64, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1284844663, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 44, 245, 119, 44, 149, 76], OperandSize::Qword)
}

