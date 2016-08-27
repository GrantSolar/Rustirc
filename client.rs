use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write, BufWriter};

fn main()
{
	let stream = TcpStream::connect("irc.freenode.org:6667").unwrap();
	let mut line = String::new();
	let mut reader = BufReader::new(&stream);
	let mut writer = BufWriter::new(&stream);
	
	let _ = writer.write(b"NICK aapple\r\n");
	let _ = writer.write(b"USER aapple tolmoon tolsun :An Apple\r\n");
	let _ = writer.write(b"JOIN ##rust\r\n");

	let sent = writer.flush();
	match sent {
		Ok(v) => println!("{:?}", v),
		Err(e) => println!("{:?}", e),
	}
	
	loop {
		let _ = reader.read_line( &mut line);
        println!("{}", line);
		line = String::new();
	}
}
