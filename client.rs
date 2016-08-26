use std::net::TcpStream;
use std::io::{BufReader, BufRead};

fn main()
{
	let stream = TcpStream::connect("irc.freenode.org:6667").unwrap();
	let mut line = String::new();
	let mut reader = BufReader::new(stream);
	loop {
		let _ = reader.read_line( &mut line);
        println!("{}", line);
		line = String::new();
	}
}
