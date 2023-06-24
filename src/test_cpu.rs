#[test]
fn test_0xa9_lda() {
    let mut cpu = crate::cpu::CPU::new();

    cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_0xa9_lda_from_memory() {
    let mut cpu = crate::cpu::CPU::new();

    use crate::cpu::Mem;
    cpu.mem_write(0x10, 0x55);    

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55)
}

#[test]
fn test_0xaa_tax() {
    let mut cpu = crate::cpu::CPU::new();

    cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0x0A)
}

#[test]
fn test_0xe8_inx() {
    let mut cpu = crate::cpu::CPU::new();

    cpu.load_and_run(vec![0xa9, 0x04, 0xaa, 0xE8, 0x00]);

    assert_eq!(cpu.register_x, 5)
}

#[test]
fn test_0xe8_inx_overflow() {
    let mut cpu = crate::cpu::CPU::new();

    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_5_ops_together() {
    let mut cpu = crate::cpu::CPU::new();

    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}
