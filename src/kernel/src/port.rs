use core::arch::asm;

/// A certain port number
pub struct HardwarePort(pub u16);

macro_rules! port_write {
    ($port: expr, $value: expr, $reg: tt) => {
        asm!(
            concat!("out dx, ", $reg), 
            in("dx") $port, 
            in($reg) $value,
            options(preserves_flags, nomem, nostack)
        )
    };
}

macro_rules! port_read {
    ($port: expr, $reg: tt) => {{
        let out;
        asm!(
            concat!(concat!("in ", $reg), ", dx"), 
            out($reg) out,
            in("dx") $port, 
            options(preserves_flags, nomem, nostack)
        );
        out
    }};
}

impl HardwarePort {
    pub unsafe fn write_u8(&self, x: u8) {
        port_write!(self.0, x, "al");
    }

    pub unsafe fn write_u16(&self, x: u16) {
        port_write!(self.0, x, "ax");
    }

    pub unsafe fn write_u32(&self, x: u32) {
        port_write!(self.0, x, "eax");
    }

    pub unsafe fn read_u8(&self) -> u8 {
        port_read!(self.0, "al")
    }

    pub unsafe fn read_u16(&self) -> u16 {
        port_read!(self.0, "ax")
    }

    pub unsafe fn read_u32(&self) -> u32 {
        port_read!(self.0, "eax")
    }
}