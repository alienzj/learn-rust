enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr_ {
    kind: IpAddrKind,
    address: String,
}

fn route(_ip_kind: IpAddrKind) {
   
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        //
    }
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home_ = IpAddr_ {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback_ = IpAddr_ {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = _x + _y;
}
