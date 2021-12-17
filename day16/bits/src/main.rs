use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn get_hex(ascii: &u8) -> u8 {
    if *ascii <= b'9' {
        return ascii - b'0';
    }

    return ascii - b'A' + 10;
}


struct BitStream {
    buff: BufReader<File>,
    curr_nibble: u8,
    bit_pos: i8,
    total_bit_pos: usize,
}

impl BitStream {
    pub fn new(filename: &str) -> BitStream {
        let f = File::open(filename).expect("Can't open file");

        BitStream {
            buff: BufReader::new(f),
            curr_nibble: 0,
            bit_pos: -1,
            total_bit_pos: 0,
        }
    }

    pub fn get_bit_pos(&self) -> usize {
        self.total_bit_pos
    }

    pub fn read_bits(&mut self, n: usize) -> io::Result<(usize, usize)> {
        let mut result = 0usize;

        for _ in 0..n {
            if self.bit_pos == -1 {
                let mut tmp = [0u8];
                let num_bytes = self.buff.read(&mut tmp)?;
                if num_bytes == 0 || tmp[0] < b'0' {
                    return Err(io::Error::new(ErrorKind::UnexpectedEof, "End of stream"));
                }
//                println!("Got byte: {} = {}", tmp[0], tmp[0] as char);
                self.curr_nibble = get_hex(&tmp[0]);
//                println!("Tranlated to {} = {:b}", self.curr_nibble, self.curr_nibble);
                self.bit_pos = 3
            }

            result = (result << 1) + (((1 << self.bit_pos) & self.curr_nibble as usize) >> self.bit_pos);
//            println!("Bit: {}; result: {}", self.bit_pos, result);
            self.bit_pos -= 1;
            self.total_bit_pos += 1;
        }

        Ok((result, 0))
    }

    fn process_literal(&mut self) -> io::Result<(usize, usize)> {
        let mut last_group = false;
        let mut result = 0;

        while !last_group {
            let last_group_bit = self.read_bits(1)?;
            last_group = last_group_bit.0 == 0;

            let digit = self.read_bits(4)?.0;
//            println!("Got digit {}", digit);
            result = (result << 4) + digit;
//            result = result * 10 + self.read_bits(4)?;
        }

        Ok((result, 0))
    }

    fn process_bit_len(&mut self, num_bits: usize) -> io::Result<(usize, usize)> {
        let starting_bit_pos = self.get_bit_pos();
//        println!("Starting bit pos: {}", starting_bit_pos);

        let mut total_result = 0;
        let mut total_version = 0;

        while self.get_bit_pos() < starting_bit_pos + num_bits {
            let (result, version) = self.process_packet()?;
            total_result += result;
            total_version += version;
//            println!("version is {}", version);
        }

        Ok((total_result, total_version))
    }

    fn process_num_packets(&mut self, num_packets: usize) -> io::Result<(usize, usize)> {
        let mut total_result = 0;
        let mut total_version = 0;

        for _ in 0..num_packets {
            let (result, version) = self.process_packet()?;
            total_result += result;
            total_version += version;
        }

        Ok((total_result, total_version))
    }

    fn process_operator(&mut self) -> io::Result<(usize, usize)> {
        let length_type = self.read_bits(1)?.0;

        if length_type == 0 {
            let num_bits = self.read_bits(15)?.0;
//            println!("{} bits", num_bits);
            self.process_bit_len(num_bits)
        } else {
            let num_packets = self.read_bits(11)?.0;
//            println!("{} packets", num_packets);
            self.process_num_packets(num_packets)
        }
    }


    fn process_packet(&mut self) -> io::Result<(usize, usize)> {
        let version = self.read_bits(3)?.0;
        let type_id = self.read_bits(3)?.0;

//        println!("version: {}; type: {}", version, type_id);
        let result = match type_id {
            4 => self.process_literal()?,
            _ => self.process_operator()?,
        };

        Ok((result.0, version + result.1))
    }

}

fn main() {
    let mut stream = BitStream::new("input.txt");

    let mut total_version = 0;

    loop {
        if let Ok(result) = stream.process_packet() {
            total_version += result.1;
            println!("result = {}, version = {}", result.0, result.1);
        } else {
            break;
        }
    }

    println!("Sum of version numbers is {}", total_version);
}
