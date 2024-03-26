use crate::port::HardwarePort;

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

pub const PIC1_OFFSET: u8 = 32u8;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

const  PIC1_CMD: HardwarePort = HardwarePort(0x20);
const PIC1_DATA: HardwarePort = HardwarePort(0x21);
const  PIC2_CMD: HardwarePort = HardwarePort(0xA0);
const PIC2_DATA: HardwarePort = HardwarePort(0xA1);

pub unsafe fn init_chained_pic8259() {
    // send garbage data to 0x80 to delay cpu
    // this allows PICs to initialize
    let dumb_wait = || HardwarePort(0x80).write_u32(0);

    let pic1_mask = PIC1_DATA.read_u8();
    let pic2_mask = PIC1_DATA.read_u8();

    // send CMD_INIT to pics
    PIC1_CMD.write_u8(CMD_INIT);
    dumb_wait();
    PIC2_CMD.write_u8(CMD_INIT);
    dumb_wait();

    // tell pics interrupt offsets
    PIC1_DATA.write_u8(PIC1_OFFSET);
    dumb_wait();
    PIC2_DATA.write_u8(PIC2_OFFSET);
    dumb_wait();

    // chaining
    PIC1_DATA.write_u32(4);
    dumb_wait();
    PIC2_DATA.write_u32(2);
    dumb_wait();

    // 8086 mode
    PIC1_DATA.write_u8(0x1);
    dumb_wait();
    PIC2_DATA.write_u8(0x1);
    dumb_wait();

    PIC1_DATA.write_u8(pic1_mask);
    PIC2_DATA.write_u8(pic2_mask);
}

pub unsafe fn notify_end_of_interrupt(interrupt_id: u8) {
    if PIC2_OFFSET <= interrupt_id && interrupt_id < PIC2_OFFSET + 8 {
        PIC2_CMD.write_u8(CMD_END_OF_INTERRUPT);
    }
    PIC1_CMD.write_u8(CMD_END_OF_INTERRUPT);
}