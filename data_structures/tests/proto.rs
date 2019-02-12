use witnet_data_structures::{proto::ProtobufConvert, types, types::IpAddress};

#[test]
fn address_proto() {
    // Serialize
    let addressv4 = types::Address {
        ip: IpAddress::Ipv4 { ip: 0x10203040 },
        port: 21337,
    };
    let address_bytes = addressv4.to_pb_bytes().unwrap();

    // Deserialize
    let address2v4 = types::Address::from_pb_bytes(&address_bytes).unwrap();

    assert_eq!(addressv4, address2v4);

    let addressv6 = types::Address {
        ip: IpAddress::Ipv6 {
            ip0: 0x10203040,
            ip1: 0xabcd,
            ip2: 0x21,
            ip3: 0x11111111,
        },
        port: 21337,
    };
    let address_bytes = addressv6.to_pb_bytes().unwrap();

    let address2v6 = types::Address::from_pb_bytes(&address_bytes).unwrap();

    assert_eq!(addressv6, address2v6);
}
