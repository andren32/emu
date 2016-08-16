// Memory: Generic memory trait which will be used as the interface
// to memory mapped peripheral devices

pub trait Memory {
    fn rb(&self, u16) -> u8;
    fn wb(&mut self, u16, u8);

    // default implementation reads and write two bytes
    // as little endian
    fn rw(&self, addr: u16) -> u16 {
        let high: u16 = (self.rb(addr) as u16) << 8;
        let low: u16 = self.rb(addr+1) as u16;
        high | low
    }

    fn ww(&mut self, addr: u16, data: u16) {
        let high: u8 = (data >> 8) as u8; // casting truncates
        let low: u8 = data as u8;
        self.wb(addr, high);
        self.wb(addr+1, low);
    }
}
