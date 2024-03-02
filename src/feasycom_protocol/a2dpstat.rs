super::indication_macros::define_enum!(A2dpStat,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
    b"3" => Connected,
    b"4" => Streaming,
);
