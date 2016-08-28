use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write, BufWriter};
use std::time::Duration;

/*fn send_line(writer : Write, text : &[u8])
{

}*/

fn main()
{
	let stream = TcpStream::connect("irc.freenode.org:6667").unwrap();
	
	//read_line is blocking so set the timeout to 1s
	let timeout : Option<Duration> = Some(Duration::new(1, 0));
	let _ = stream.set_read_timeout(timeout);

	let mut line = String::new();
	let mut reader = BufReader::new(&stream);
	let mut writer = BufWriter::new(&stream);
	
	let _ = writer.write(b"NICK aapple\r\n");
	let _ = writer.write(b"USER aapple tolmoon tolsun :An Apple\r\n");
	let _ = writer.write(b"JOIN ##test\r\n");
	let _ = writer.write(b"PRIVMSG ##test :Test msg here\r\n");

	let sent = writer.flush();
	match sent {
		Ok(v) => println!("{:?}", v),
		Err(e) => println!("{:?}", e),
	}

	let send = String::new();

	loop {
		let read = reader.read_line( &mut line);
		match read {
			Ok(0) => {},
			Ok(v) => println!("{}", line),
			Err(e) => {},//println!("ERROR - {}", e),
		}
		line = String::new();
	}
}
