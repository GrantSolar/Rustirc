use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write, BufWriter};
use std::time::Duration;

fn send_line<T : Write> (mut writer : T, text : &[u8])
{
	println!("{:?}", text);
	let _ = writer.write(text);
	let _ = writer.flush();
}

fn process_line<T : Write>(mut writer : T, line : String)
{
	println!("{}", line);
	//Split line by ":"s and store in a vector
	let split : Vec<&str> = line.split(":").collect();

	match split[0] {
		"" => {},
		"PING " => {
			send_line(&mut writer, b"PONG");
		},
		_ => {},
	}
}

fn main()
{
	let stream = TcpStream::connect("irc.freenode.org:6667").unwrap();
	
	//read_line is blocking so set the timeout to 1s
	//let timeout : Option<Duration> = Some(Duration::new(1, 0));
	//let _ = stream.set_read_timeout(timeout);

	let mut line = String::new();
	let mut reader = BufReader::new(&stream);
	let mut writer = BufWriter::new(&stream);
	
	let _ = writer.write(b"NICK aapple\r\n");
	let _ = writer.write(b"USER aapple tolmoon tolsun :An Apple\r\n");
	let _ = writer.write(b"JOIN ##test\r\n");
	let _ = writer.write(b"PRIVMSG ##test :Test msg here\r\n");

	let sent = writer.flush();
	match sent {
		Ok(v) => {},
		Err(e) => println!("{:?}", e),
	}/*
	//Doing things like this gives a 404 error for some reason
	send_line(&mut writer, b"NICK aapple\r\n");
	send_line(&mut writer, b"USER aapple tolmoon tolsun :An Apple\r\n");
	send_line(&mut writer, b"PRIVMSG ##test :Test msg here\r\n");*/

	send_line(&mut writer, b"PRIVMSG ##test :Test2\r\n");
	loop {
		let read = reader.read_line( &mut line);
		match read {
			Ok(0) => {},
			Ok(v) => {
				process_line(&mut writer, line)},
			Err(e) => println!("ERROR - {}", e),
		}
		line = String::new();
	}
}
