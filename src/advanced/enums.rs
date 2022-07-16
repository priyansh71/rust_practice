// enum IpAddressKind{
//     v4,
//     v6
// }

enum IpAddressKind{
    v4(u8,u8,u8,u8),
    v6(String)
}

struct IPAddress{
    kind: IpAddressKind,
    address: String
}

pub fn fun(){
    let four = IpAddressKind::v4;
    let six = IpAddressKind::v6;

    let localhost = IpAddressKind::v4(127,0,0,1);

}