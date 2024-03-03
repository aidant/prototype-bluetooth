use alloc::vec::Vec;
use defmt::Format;

mod macros {
    macro_rules! define_bytes {
        ($($mod:ident => $bytes:literal,)*) => {
            $(mod $mod {
                use alloc::vec::Vec;

                pub fn stringify() -> Vec<u8> {
                    $bytes.to_vec()
                }
            })*
        };
    }

    pub(super) use define_bytes;
    pub(super) use define_commands;
}

macros::define_bytes!(
    ver => b"AT+VER",
    addr => b"AT+ADDR",
    stat => b"AT+STAT",
    dsca => b"AT+DSCA",
    reboot => b"AT+REBOOT",
    restore => b"AT+RESTORE",
    closeat => b"AT+CLOSEAT",
    a2dpstat => b"AT+A2DPSTAT",
    a2dpdisc => b"AT+A2DPDISC",
    a2dpdec => b"AT+A2DPDEC",
    playpause => b"AT+PLAYPAUSE",
    play => b"AT+PLAY",
    pause => b"AT+PAUSE",
    stop => b"AT+STOP",
    forward => b"AT+FORWARD",
    backward => b"AT+BACKWARD",
    sppstat => b"AT+SPPSTAT",
    sppdisc => b"AT+SPPDISC",
    gattstat => b"AT+GATTSTAT",
    gattdisc => b"AT+GATTDISC",
);

macro_rules! commands {
	($($type:ident$(($params:tt))?),+ $(,)?) => {
		#[derive(Debug, Eq, PartialEq, Clone, Format)]
		pub enum Command {
		  Test,

		  $($type$(($params))?,)*
		}

		impl Into<Vec<u8>> for Command {
			fn into(self) -> Vec<u8> {
				match self {
					Self::Test => b"AT".to_vec(),

					$(Self::$type$(($params))? => $type::try_from($($params)?),)*
				}
			}
		}
	};
}

commands!(
    Ver,
    Addr,
    LeAddr,
    Name((name, enable_suffix)),
    LeName,
    LeCfg,
    Baud,
    UartCfg,
    Pin,
    Ssp,
    Cod,
    Plist,
    TpMode,
    Stat,
    AutoConn,
    Scan,
    InqCfg,
    SpkVol,
    I2sCfg,
    SpdifCfg,
    Dsca,
    Reboot,
    Restore,
    CloseAt,
    BtEn,
    Pair,
    A2dpStat,
    A2dpRole,
    A2dpConn,
    A2dpDisc,
    A2dpDec,
    AvrcpCfg,
    PlayPause,
    Play,
    Pause,
    Stop,
    Forward,
    Backward,
    SppStat,
    SppConn,
    SppDisc,
    SppSend,
    GattStat,
    GattDisc,
    GattSend,
);

// #[derive(Debug, Eq, PartialEq, Clone, Format)]
// pub enum Command {
//     Test,

//     Ver,
//     Addr,
//     LeAddr,
//     Name,    // TODO params String Boolean
//     LeName,  // TODO params String Boolean
//     LeCfg,   // TODO params Boolean
//     Baud,    // TODO params Baudrate
//     UartCfg, // TODO params Boolean
//     Pin,     // TODO params String
//     Ssp,     // TODO params Boolean
//     Cod,     // TODO params String
//     Plist,   // TODO params Custom
//     TpMode,  // TODO params Boolean
//     Stat,
//     AutoConn, // TODO params Custom
//     Scan,     // TODO params Boolean
//     InqCfg,   // TODO params Boolean
//     SpkVol,   // TODO params Custom
//     I2sCfg,   // TODO params Custom
//     SpdifCfg, // TODO params Custom
//     Dsca,
//     Reboot,
//     Restore,
//     CloseAt,
//     BtEn, // TODO params Boolean
//     Pair, // TODO params Boolean

//     A2dpStat,
//     A2dpRole, // TODO params Boolean
//     A2dpConn, // TODO params String
//     A2dpDisc,
//     A2dpDec,
//     AvrcpCfg, // TODO params Custom
//     PlayPause,
//     Play,
//     Pause,
//     Stop,
//     Forward,
//     Backward,

//     SppStat,
//     SppConn, // TODO params String
//     SppDisc,
//     SppSend, // TODO params usize String

//     GattStat,
//     // does GattConn exist?
//     GattDisc,
//     GattSend, // TODO params usize String
// }
