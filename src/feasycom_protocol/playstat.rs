super::indication_macros::define_enum!(PlayStat,
    b"0" => Stopped,
    b"1" => Playing,
    b"2" => Paused,
    b"3" => FastForwarding,
    b"4" => FastRewinding,
);
