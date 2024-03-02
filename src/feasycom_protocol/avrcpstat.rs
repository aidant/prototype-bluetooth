super::indication_macros::define_enum!(AvrcpStat,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
    b"3" => Connected,
);
