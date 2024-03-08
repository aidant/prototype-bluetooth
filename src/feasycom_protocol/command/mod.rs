use alloc::string::{String, ToString};
use core::fmt::{self, Display};
use defmt::Format;

macro_rules! command_with_required_parameters {
    ($($type:ident => $command:literal),+ $(,)?) => {
        $(
            #[derive(Debug, Eq, PartialEq, Clone, Format)]
            pub struct $type<const STATE: u8 = 0> {
                str: String,
            }

            impl $type<0> {
                pub fn new() -> $type<1> {
                    $type {
                        str: $command.to_string(),
                    }
                }
            }
        )+
    };
}

macro_rules! command_can_format {
    ($type:ident) => {
        impl<const STATE: u8> $type<STATE> {
            pub fn as_bytes(&mut self) -> &[u8] {
                self.str.push_str("\r\n");
                &self.str.as_bytes()
            }
        }

        impl<const STATE: u8> Display for $type<STATE> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.str)
            }
        }
    };
    ($type:ident, $state:literal) => {
        impl $type<$state> {
            pub fn as_bytes(&mut self) -> &[u8] {
                self.str.push_str("\r\n");
                &self.str.as_bytes()
            }
        }

        impl Display for $type<$state> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.str)
            }
        }
    };
}

macro_rules! command_with_optional_parameters {
    ($($type:ident => $command:literal),+ $(,)?) => {
        command_with_required_parameters!($($type => $command),+);

        $(
            command_can_format!($type);
        )+
    };
}

command_with_required_parameters!(
    HfpDtmf => "AT+HFPDTMF",
    PbDown => "AT+PBDOWN",
    SppConn => "AT+SPPCONN",
    SppSend => "AT+SPPSEND",
    GattSend => "AT+GATTSEND",
);

command_with_optional_parameters!(
    Ver => "AT+VER",
    Addr => "AT+ADDR",
    LeAddr => "AT+LEADDR",
    Name => "AT+NAME",
    LeName => "AT+NAME",
    LeCfg => "AT+LECFG",
    Baud => "AT+BAUD",
    UartCfg => "AT+UARTCFG",
    Pin => "AT+PIN",
    Spp => "AT+SPP",
    Cod => "AT+COD",
    PList => "AT+PLIST",
    TpMode => "AT+TPMODE",
    Stat => "AT+STAT",
    AutoConn => "AT+AUTOCONN",
    Scan => "AT+SCAN",
    InqCfg => "AT+INQCFG",
    SpkVol => "AT+SPKVOL",
    I2sCfg => "AT+I2SCFG",
    SpdifCfg => "AT+SPDIFCFG",
    Dsca => "AT+DSCA",
    Reboot => "AT+REBOOT",
    Restore => "AT+RESTORE",
    CloseAt => "AT+CLOSEAT",
    BtEn => "AT+BTEN",
    Pair => "AT+PAIR",
    HfpStat => "AT+HFPSTAT",
    HfpConn => "AT+HFPCONN",
    HfpDisc => "AT+HFPDISC",
    HfpDial => "AT+HFPDIAL",
    HfpAnsw => "AT+HFPANSW",
    HfpChup => "AT+HFPCHUP",
    HfpAdts => "AT+HFPADTS",
    MuteMic => "AT+MUTEMIC",
    A2dpStat => "AT+A2DPSTAT",
    A2dpRole => "AT+A2DPROLE",
    A2dpConn => "AT+A2DPCONN",
    A2dpDisc => "AT+A2DPDISC",
    A2dpDec => "AT+A2DPDEC",
    AvrcpCfg => "AT+AVRCPCFG",
    PlayPause => "AT+PLAYPAUSE",
    Play => "AT+PLAY",
    Pause => "AT+PAUSE",
    Stop => "AT+STOP",
    Forward => "AT+FORWARD",
    Backward => "AT+BACKWARD",
    SppStat => "AT+SPPSTAT",
    SppDisc => "AT+SPPDISC",
    GattStat => "AT+GATTSTAT",
    GattDisc => "AT+GATTDISC",
);

macro_rules! command_param_str {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: &str) -> $type<$to> {
                self.str.push($separator);
                self.str.push_str($name);

                $type { str: self.str }
            }
        }
    };
}

macro_rules! command_param_literal {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident, $literal:literal) => {
        impl $type<$from> {
            pub fn $name(mut self) -> $type<$to> {
                self.str.push($separator);
                self.str.push_str($literal);

                $type { str: self.str }
            }
        }
    };
}

macro_rules! command_param_bool {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: bool) -> $type<$to> {
                self.str.push($separator);
                self.str.push(if $name { '1' } else { '0' });

                $type { str: self.str }
            }
        }
    };
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident, inverted) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: bool) -> $type<$to> {
                self.str.push($separator);
                self.str.push(if $name { '0' } else { '1' });

                $type { str: self.str }
            }
        }
    };
}

macro_rules! command_param_number {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident, $size:ty) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: $size) -> $type<$to> {
                self.str.push($separator);
                self.str.push_str($name.to_string().as_str());

                $type { str: self.str }
            }
        }
    };
}

macro_rules! command_param_data {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: &str) -> $type<$to> {
                self.str.push($separator);
                self.str.push_str($name.len().to_string().as_str());
                self.str.push(',');
                self.str.push_str($name);

                $type { str: self.str }
            }
        }
    };
}

command_param_str!(Name, 1 -> 2, '=', name);
command_param_bool!(Name, 2 -> 3, ',', enable_suffix);

command_param_str!(LeName, 1 -> 2, '=', le_name);
command_param_bool!(LeName, 2 -> 3, ',', enable_suffix);

command_param_bool!(LeCfg, 1 -> 2, '=', enable_random_address);

command_param_number!(Baud, 1 -> 2, '=', baudrate, u32); // TODO baudrate enum

command_param_bool!(UartCfg, 1 -> 2, '=', enable_cts_rts);

command_param_str!(Pin, 1 -> 2, '=', pin); // TODO min and max length

command_param_bool!(Spp, 1 -> 2, '=', enable_simple_paring);

command_param_str!(Cod, 1 -> 2, '=', class_of_device); // TODO class of device builder

command_param_literal!(PList, 1 -> 2, '=', clear_paired_all, "0");
command_param_number!(PList, 1 -> 2, '=', clear_paired_index, u8); // TODO 1-8 index
command_param_str!(PList, 1 -> 2, '=', clear_paired_mac);

command_param_bool!(TpMode, 1 -> 2, '=', enable_throughput_mode);

command_param_literal!(AutoConn, 1 -> 2, '=', disable_auto_connection, "0");
command_param_number!(AutoConn, 1 -> 2, '=', attempts, u8);

command_param_literal!(Scan, 1 -> 2, '=', start, "1");
command_param_literal!(Scan, 1 -> 2, '=', stop, "0");

command_param_bool!(InqCfg, 1 -> 2, '=', enable_auto_scan);

command_param_literal!(SpkVol, 1 -> 2, '=', increase, "+");
command_param_literal!(SpkVol, 1 -> 2, '=', decrease, "-");

command_param_number!(I2sCfg, 1 -> 2, '=', configure_i2s_pcm, u8); // TODO i2s/pcm builder

command_param_bool!(SpdifCfg, 1 -> 2, '=', enable_spdif);

command_param_bool!(BtEn, 1 -> 2, '=', enable_bluetooth, inverted);

command_param_bool!(Pair, 1 -> 2, '=', enable_pairing, inverted);

command_param_str!(HfpConn, 1 -> 2, '=', mac);

command_param_str!(HfpDial, 1 -> 2, '=', phone_number);

command_param_str!(HfpDtmf, 1 -> 2, '=', code); // TODO [0-9#*]
command_can_format!(HfpDtmf, 2);

command_param_literal!(HfpAdts, 1 -> 2, '=', transfer_to_remote, "0");
command_param_literal!(HfpAdts, 1 -> 2, '=', transfer_from_remote, "1");

command_param_literal!(MuteMic, 1 -> 2, '=', mute, "1");
command_param_literal!(MuteMic, 1 -> 2, '=', unmute, "0");

command_param_literal!(A2dpRole, 1 -> 2, '=', slave, "0");
command_param_literal!(A2dpRole, 1 -> 2, '=', master, "1");

command_param_str!(A2dpConn, 1 -> 2, '=', mac);

command_param_number!(AvrcpCfg, 1 -> 2, '=', avrcpcfg, u8); // TODO avrcpcfg builder

command_param_number!(PbDown, 1 -> 2, '=', phonebook, u8); // TODO 0-5
command_can_format!(PbDown, 2);
command_param_number!(PbDown, 2 -> 3, ',', max_items, u16);
command_can_format!(PbDown, 3);

command_param_str!(SppConn, 1 -> 2, '=', mac);
command_can_format!(SppConn, 2);

command_param_data!(SppSend, 1 -> 2, '=', payload);
command_can_format!(SppSend, 2);

command_param_data!(GattSend, 1 -> 2, '=', payload);
command_can_format!(GattSend, 2);
