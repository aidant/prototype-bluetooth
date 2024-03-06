use alloc::string::{String, ToString};
use core::fmt::{self, Display};
use defmt::Format;

macro_rules! command {
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
        )+
    };
}

command!(
    Ver => "AT+VER",
    Addr => "AT+ADDR",
    LeAddr => "AT+LEADDR",
    Name => "AT+NAME",
    LeName => "AT+NAME",
    LeCfg => "AT+LECFG",
    Baud => "AT+BAUD",
    // UARTCFG
    // PIN
    // SSP
    // COD
    // PLIST
    // TPMODE
    Stat => "AT+STAT",
    // AUTOCONN
    // SCAN
    // INQCFG
    // SPKVOL
    // I2SCFG
    // SPDIFCFG
    Dsca => "AT+DSCA",
    Reboot => "AT+REBOOT",
    Restore => "AT+RESTORE",
    CloseAt => "AT+CLOSEAT",
    // BTEN
    // PAIR
    HfpStat => "AT+HFPSTAT",
    // HFPCONN
    HfpDisc => "AT+HFPDISC",
    // HFPDIAL
    // HFPDTMF
    HfpAnsw => "AT+HFPANSW",
    HfpChup => "AT+HFPCHUP",
    // HFPADTS
    // MUTEMIC
    A2dpStat => "AT+A2DPSTAT",
    // A2DPROLE
    // A2DPCONN
    A2dpDisc => "AT+A2DPDISC",
    A2dpDec => "AT+A2DPDEC",
    // AVRCPCFG
    Playpause => "AT+PLAYPAUSE",
    Play => "AT+PLAY",
    Pause => "AT+PAUSE",
    Stop => "AT+STOP",
    Forward => "AT+FORWARD",
    Backward => "AT+BACKWARD",
    // PBDOWN
    SppStat => "AT+SPPSTAT",
    // SPPCONN
    SppDisc => "AT+SPPDISC",
    // SPPSEND
    GattStat => "AT+GATTSTAT",
    GattDisc => "AT+GATTDISC",
    // GATTSEND
);

macro_rules! command_param_str {
    ($type:ident, $from:literal -> $to:literal, $separator:literal, $name:ident) => {
        impl $type<$from> {
            pub fn $name(mut self, $name: &str) -> $type<$to> {
                self.str.push($separator);
                self.str.push_str($name);

                unsafe { core::mem::transmute(self) }
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

                unsafe { core::mem::transmute(self) }
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

                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

command_param_str!(Name, 1 -> 2, '=', name);
command_param_bool!(Name, 2 -> 3, ',', enable_suffix);

command_param_str!(LeName, 1 -> 2, '=', le_name);
command_param_bool!(LeName, 2 -> 3, ',', enable_suffix);

command_param_bool!(LeCfg, 1 -> 2, '=', enable_random_address);

command_param_number!(Baud, 1 -> 2, '=', baudrate, u32);
