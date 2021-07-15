use bit_field::BitField;
use core;
use core::ptr::{read_volatile};
use volatile::Volatile;
use rustduino::atmega2560p::{usart_initialize,usart_initialize::Usart};

impl Usart{
   pub fn recieve_enable(&mut self){
    unsafe {
        self.stctrlh.update(|ctrl| {
            ctrl.set_bit(4, true);
        });
    }
   }
   pub fn recieve_data(&mut self)->Option<Volatile<u8>,Volatile<u32>>{
    unsafe{
        let  ucsrc=read_volatile(&self.ucsrc);
        let  ucsrb=read_volatile(&self.ucsrb);
        if ucsrc.gets_bits(1..3)==0b11 && ucsrb.get_bit(2){
            let ucsra=read_volatile(&self.ucsra);
            while !(ucsra.getbit(7)){};
            let ucsra=read_volatile(&self.ucsra);
            let ucsrb=read_volatile(&self.ucsrb);
            let udr=read_volatile(&mut self.udr);
            if ucsra.get_bits(2..5)!=0b000{
                Some(-1)
            }
            else{
                let rxb8=ucsrb.get_bits(1..2);
                udr=udr.set_bits(8..9,rxb8);
                Some(udr);
            }
        }
        else{
            let ucsra=read_volatile(&self.ucsra);
            while !(ucsra.getbit(7)){};
           if error_check(&mut self){
               Some(-1)
           }
           if parity_check(&mut self){
              Some(-1)
           }
            Some(read_volatile(&mut self.udr))
            
        }
    }
}
  pub fn error_check(&mut self)->bool{
      let ucsra=read_volatile(&self.ucsra);
      if ucsra.getbits(2..4)!=0b000{
          true
      }
      else{
          false
      }
  }
  pub fn parity_check(&mut self)->bool{
    let ucsra=read_volatile(&self.ucsra);
    if ucsra.getbit(4)==0b1{
        true
    } 
    else{
        false
    }
  }
}