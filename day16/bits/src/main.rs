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

    pub fn read_bits(&mut self, n: usize) -> io::Result<usize> {
        let mut result = 0usize;

        for _ in 0..n {
            if self.bit_pos == -1 {
                let mut tmp = [0u8];
                let num_bytes = self.buff.read(&mut tmp)?;
                if num_bytes == 0 || tmp[0] < b'0' {
                    return Err(io::Error::new(ErrorKind::UnexpectedEof, "End of stream"));
                }
                self.curr_nibble = get_hex(&tmp[0]);
                self.bit_pos = 3
            }

            result = (result << 1) + (((1 << self.bit_pos) & self.curr_nibble as usize) >> self.bit_pos);
            self.bit_pos -= 1;
            self.total_bit_pos += 1;
        }

        Ok(result)
    }

    fn process_literal(&mut self) -> io::Result<Vec<usize>> {
        let mut last_group = false;
        let mut result = 0;

        while !last_group {
            let last_group_bit = self.read_bits(1)?;
            last_group = last_group_bit == 0;

            let digit = self.read_bits(4)?;
            result = (result << 4) + digit;
        }

        Ok(vec![result])
    }

    fn process_bit_len(&mut self, num_bits: usize) -> io::Result<Vec<usize>> {
        let starting_bit_pos = self.get_bit_pos();

        let mut result:Vec<usize> = Vec::new();

        while self.get_bit_pos() < starting_bit_pos + num_bits {
            result.append(&mut self.process_packet()?);
        }

        Ok(result)
    }

    fn process_num_packets(&mut self, num_packets: usize) -> io::Result<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();

        for _ in 0..num_packets {
            result.append(&mut self.process_packet()?);
        }

        Ok(result)
    }

    fn process_operator(&mut self) -> io::Result<Vec<usize>> {
        let length_type = self.read_bits(1)?;

        if length_type == 0 {
            let num_bits = self.read_bits(15)?;
            self.process_bit_len(num_bits)
        } else {
            let num_packets = self.read_bits(11)?;
            self.process_num_packets(num_packets)
        }
    }


    fn process_packet(&mut self) -> io::Result<Vec<usize>> {
        let _version = self.read_bits(3)?;
        let type_id = self.read_bits(3)?;

        let result = match type_id {
            4 => self.process_literal()?[0],
            0 => self.process_operator()?
                            .iter()
                            .sum(),
            1 => self.process_operator()?
                            .iter()
                            .fold(1, |prod, x| prod * x),
            2 => *self.process_operator()?
                            .iter()
                            .min()
                            .unwrap(),
            3 => *self.process_operator()?
                            .iter()
                            .max()
                            .unwrap(),
            5 | 6 | 7 => {
                let result = self.process_operator()?;
                match type_id {
                    5 => if result[0] > result[1] { 1 } else { 0 },
                    6 => if result[0] < result[1] { 1 } else { 0 },
                    7 => if result[0] == result[1] { 1 } else { 0 },
                    _ => panic!("Impossible to get here"),
                }
            },

            _ => panic!("Unknown operator"),
        };

        Ok(vec![result])
    }

}

fn main() {
    let mut stream = BitStream::new("input.txt");

    let mut answer:Vec<usize> = Vec::new();

    loop {
        if let Ok(mut result) = stream.process_packet() {
            answer.append(&mut result);
        } else {
            break;
        }
    }

    println!("Answer is {}", answer[0]);
}
