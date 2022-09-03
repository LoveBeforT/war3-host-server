// credit goes to Ghost++ project

pub const W3GS_HEADER_CONSTANTT:u8 = 247;   // 0xf7

pub const W3GS_PING_FROM_HOST:u8 = 1;	    // 0x01
pub const W3GS_SLOTINFOJOIN:u8 = 4;	        // 0x04
pub const W3GS_REJECTJOIN:u8 = 5;	        // 0x05
pub const W3GS_PLAYERINFO:u8 = 6;	        // 0x06
pub const W3GS_PLAYERLEAVE_OTHERS:u8 = 7;	// 0x07
pub const W3GS_GAMELOADED_OTHERS:u8 = 8;	// 0x08
pub const W3GS_SLOTINFO:u8 = 9;	            // 0x09
pub const W3GS_COUNTDOWN_START:u8 = 10;	    // 0x0A
pub const W3GS_COUNTDOWN_END:u8 = 11;	    // 0x0B
pub const W3GS_INCOMING_ACTION:u8 = 12;	    // 0x0C
pub const W3GS_CHAT_FROM_HOST:u8 = 15;	    // 0x0F
pub const W3GS_START_LAG:u8 = 16;	        // 0x10
pub const W3GS_STOP_LAG:u8 = 17;	        // 0x11
pub const W3GS_HOST_KICK_PLAYER:u8 = 28;	// 0x1C
pub const W3GS_REQJOIN:u8 = 30;	            // 0x1E
pub const W3GS_LEAVEGAME:u8 = 33;	        // 0x21
pub const W3GS_GAMELOADED_SELF:u8 = 35;	    // 0x23
pub const W3GS_OUTGOING_ACTION:u8 = 38; 	// 0x26
pub const W3GS_OUTGOING_KEEPALIVE:u8 = 39;	// 0x27
pub const W3GS_CHAT_TO_HOST:u8 = 40;	    // 0x28
pub const W3GS_DROPREQ:u8 = 41;	            // 0x29
pub const W3GS_SEARCHGAME:u8 = 47;	        // 0x2F (UDP/LAN)
pub const W3GS_GAMEINFO:u8 = 48;	        // 0x30 (UDP/LAN)
pub const W3GS_CREATEGAME:u8 = 49;	        // 0x31 (UDP/LAN)
pub const W3GS_REFRESHGAME:u8 = 50;	        // 0x32 (UDP/LAN)
pub const W3GS_DECREATEGAME:u8 = 51;	    // 0x33 (UDP/LAN)
pub const W3GS_CHAT_OTHERS:u8 = 52;	        // 0x34
pub const W3GS_PING_FROM_OTHERS:u8 = 53;	// 0x35
pub const W3GS_PONG_TO_OTHERS:u8 = 54;	    // 0x36
pub const W3GS_MAPCHECK:u8 = 61;	        // 0x3D
pub const W3GS_STARTDOWNLOAD:u8 = 63;	    // 0x3F
pub const W3GS_MAPSIZE:u8 = 66;	            // 0x42
pub const W3GS_MAPPART:u8 = 67;	            // 0x43
pub const W3GS_MAPPARTOK:u8 = 68;	        // 0x44
pub const W3GS_MAPPARTNOTOK:u8 = 69;	    // 0x45 - just a guess, received this packet after forgetting to send a crc in pub const W3GS_MAPPART (f7 45 0a 00 01 02 01 00 00 00)
pub const W3GS_PONG_TO_HOST:u8 = 70;	    // 0x46
pub const W3GS_INCOMING_ACTION2:u8 = 72;	// 0x48 - received this packet when there are too many actions to fit in pub const W3GS_INCOMING_ACTION
