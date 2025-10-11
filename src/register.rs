#![allow(unused)]

// Raspberry Pi VideoCoreIV
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D Register Address Map
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D Base Address ($20C00000 PHYSICAL, $7EC00000 BUS)
pub const V3D_BASE: u32 = 0xC00000;
/// V3D Identification 0 (V3D Block Identity)
pub const V3D_IDENT0: u32 = 0x00000;
/// V3D Identification 1 (V3D Configuration A)
pub const V3D_IDENT1: u32 = 0x00004;
/// V3D Identification 2 (V3D Configuration B)
pub const V3D_IDENT2: u32 = 0x00008;
/// V3D Identification 3 (V3D Configuration C)
pub const V3D_IDENT3: u32 = 0x0000C;
/// V3D Scratch Register
pub const V3D_SCRATCH: u32 = 0x00010;
/// V3D L2 Cache Control
pub const V3D_L2CACTL: u32 = 0x00020;
/// V3D Slices Cache Control
pub const V3D_SLCACTL: u32 = 0x00024;
/// V3D Interrupt Control
pub const V3D_INTCTL: u32 = 0x00030;
/// V3D Interrupt Enables
pub const V3D_INTENA: u32 = 0x00034;
/// V3D Interrupt Disables
pub const V3D_INTDIS: u32 = 0x00038;
/// V3D Control List Executor Thread 0 Control & Status
pub const V3D_CT0CS: u32 = 0x00100;
/// V3D Control List Executor Thread 1 Control & Status
pub const V3D_CT1CS: u32 = 0x00104;
/// V3D Control List Executor Thread 0 End Address
pub const V3D_CT0EA: u32 = 0x00108;
/// V3D Control List Executor Thread 1 End Address
pub const V3D_CT1EA: u32 = 0x0010C;
/// V3D Control List Executor Thread 0 Current Address
pub const V3D_CT0CA: u32 = 0x00110;
/// V3D Control List Executor Thread 1 Current Address
pub const V3D_CT1CA: u32 = 0x00114;
/// V3D Control List Executor Thread 0 Return Address
pub const V3D_CT0RA0: u32 = 0x00118;
/// V3D Control List Executor Thread 1 Return Address
pub const V3D_CT1RA0: u32 = 0x0011C;
/// V3D Control List Executor Thread 0 List Counter
pub const V3D_CT0LC: u32 = 0x00120;
/// V3D Control List Executor Thread 1 List Counter
pub const V3D_CT1LC: u32 = 0x00124;
/// V3D Control List Executor Thread 0 Primitive List Counter
pub const V3D_CT0PC: u32 = 0x00128;
/// V3D Control List Executor Thread 1 Primitive List Counter
pub const V3D_CT1PC: u32 = 0x0012C;
/// V3D Pipeline Control & Status
pub const V3D_PCS: u32 = 0x00130;
/// V3D Binning Mode Flush Count
pub const V3D_BFC: u32 = 0x00134;
/// V3D Rendering Mode Frame Count
pub const V3D_RFC: u32 = 0x00138;
/// V3D Current Address Of Binning Memory Pool
pub const V3D_BPCA: u32 = 0x00300;
/// V3D Remaining Size Of Binning Memory Pool
pub const V3D_BPCS: u32 = 0x00304;
/// V3D Address Of Overspill Binning Memory Block
pub const V3D_BPOA: u32 = 0x00308;
/// V3D Size Of Overspill Binning Memory Block
pub const V3D_BPOS: u32 = 0x0030C;
/// V3D Binner Debug
pub const V3D_BXCF: u32 = 0x00310;
/// V3D Reserve QPUs 0-7
pub const V3D_SQRSV0: u32 = 0x00410;
/// V3D Reserve QPUs 8-15
pub const V3D_SQRSV1: u32 = 0x00414;
/// V3D QPU Scheduler Control
pub const V3D_SQCNTL: u32 = 0x00418;
/// V3D QPU Scheduler State
pub const V3D_SQCSTAT: u32 = 0x0041C;
/// V3D QPU User Program Request Program Address
pub const V3D_SRQPC: u32 = 0x00430;
/// V3D QPU User Program Request Uniforms Address
pub const V3D_SRQUA: u32 = 0x00434;
/// V3D QPU User Program Request Uniforms Length
pub const V3D_SRQUL: u32 = 0x00438;
/// V3D QPU User Program Request Control & Status
pub const V3D_SRQCS: u32 = 0x0043C;
/// V3D VPM Allocator Control
pub const V3D_VPACNTL: u32 = 0x00500;
/// V3D VPM Base (User) Memory Reservation
pub const V3D_VPMBASE: u32 = 0x00504;
/// V3D Performance Counter Clear
pub const V3D_PCTRC: u32 = 0x00670;
/// V3D Performance Counter Enables
pub const V3D_PCTRE: u32 = 0x00674;
/// V3D Performance Counter Count 0
pub const V3D_PCTR0: u32 = 0x00680;
/// V3D Performance Counter Mapping 0
pub const V3D_PCTRS0: u32 = 0x00684;
/// V3D Performance Counter Count 1
pub const V3D_PCTR1: u32 = 0x00688;
/// V3D Performance Counter Mapping 1
pub const V3D_PCTRS1: u32 = 0x0068C;
/// V3D Performance Counter Count 2
pub const V3D_PCTR2: u32 = 0x00690;
/// V3D Performance Counter Mapping 2
pub const V3D_PCTRS2: u32 = 0x00694;
/// V3D Performance Counter Count 3
pub const V3D_PCTR3: u32 = 0x00698;
/// V3D Performance Counter Mapping 3
pub const V3D_PCTRS3: u32 = 0x0069C;
/// V3D Performance Counter Count 4
pub const V3D_PCTR4: u32 = 0x006A0;
/// V3D Performance Counter Mapping 4
pub const V3D_PCTRS4: u32 = 0x006A4;
/// V3D Performance Counter Count 5
pub const V3D_PCTR5: u32 = 0x006A8;
/// V3D Performance Counter Mapping 5
pub const V3D_PCTRS5: u32 = 0x006AC;
/// V3D Performance Counter Count 6
pub const V3D_PCTR6: u32 = 0x006B0;
/// V3D Performance Counter Mapping 6
pub const V3D_PCTRS6: u32 = 0x006B4;
/// V3D Performance Counter Count 7
pub const V3D_PCTR7: u32 = 0x006B8;
/// V3D Performance Counter Mapping 7
pub const V3D_PCTRS7: u32 = 0x006BC;
/// V3D Performance Counter Count 8
pub const V3D_PCTR8: u32 = 0x006C0;
/// V3D Performance Counter Mapping 8
pub const V3D_PCTRS8: u32 = 0x006C4;
/// V3D Performance Counter Count 9
pub const V3D_PCTR9: u32 = 0x006C8;
/// V3D Performance Counter Mapping 9
pub const V3D_PCTRS9: u32 = 0x006CC;
/// V3D Performance Counter Count 10
pub const V3D_PCTR10: u32 = 0x006D0;
/// V3D Performance Counter Mapping 10
pub const V3D_PCTRS10: u32 = 0x006D4;
/// V3D Performance Counter Count 11
pub const V3D_PCTR11: u32 = 0x006D8;
/// V3D Performance Counter Mapping 11
pub const V3D_PCTRS11: u32 = 0x006DC;
/// V3D Performance Counter Count 12
pub const V3D_PCTR12: u32 = 0x006E0;
/// V3D Performance Counter Mapping 12
pub const V3D_PCTRS12: u32 = 0x006E4;
/// V3D Performance Counter Count 13
pub const V3D_PCTR13: u32 = 0x006E8;
/// V3D Performance Counter Mapping 13
pub const V3D_PCTRS13: u32 = 0x006EC;
/// V3D Performance Counter Count 14
pub const V3D_PCTR14: u32 = 0x006F0;
/// V3D Performance Counter Mapping 14
pub const V3D_PCTRS14: u32 = 0x006F4;
/// V3D Performance Counter Count 15
pub const V3D_PCTR15: u32 = 0x006F8;
/// V3D Performance Counter Mapping 15
pub const V3D_PCTRS15: u32 = 0x006FC;
/// V3D Configure
pub const V3D_DBCFG: u32 = 0x00E00;
/// V3D S Control & Status
pub const V3D_DBSCS: u32 = 0x00E04;
/// V3D S Configure
pub const V3D_DBSCFG: u32 = 0x00E08;
/// V3D S SR
pub const V3D_DBSSR: u32 = 0x00E0C;
/// V3D SD R0
pub const V3D_DBSDR0: u32 = 0x00E10;
/// V3D SD R1
pub const V3D_DBSDR1: u32 = 0x00E14;
/// V3D SD R2
pub const V3D_DBSDR2: u32 = 0x00E18;
/// V3D SD R3
pub const V3D_DBSDR3: u32 = 0x00E1C;
/// V3D QPU Run
pub const V3D_DBQRUN: u32 = 0x00E20;
/// V3D QPU Halt
pub const V3D_DBQHLT: u32 = 0x00E24;
/// V3D QPU Step
pub const V3D_DBQSTP: u32 = 0x00E28;
/// V3D QPU Interrupt Enables
pub const V3D_DBQITE: u32 = 0x00E2C;
/// V3D QPU Interrupt Control
pub const V3D_DBQITC: u32 = 0x00E30;
/// V3D QPU GHC
pub const V3D_DBQGHC: u32 = 0x00E34;
/// V3D QPU GHG
pub const V3D_DBQGHG: u32 = 0x00E38;
/// V3D QPU GHH
pub const V3D_DBQGHH: u32 = 0x00E3C;
/// V3D PSE Error Signals
pub const V3D_DBGE: u32 = 0x00F00;
/// V3D FEP Overrun Error Signals
pub const V3D_FDBGO: u32 = 0x00F04;
/// V3D FEP Interface Ready & Stall Signals, FEP Busy Signals
pub const V3D_FDBGB: u32 = 0x00F08;
/// V3D FEP Internal Ready Signals
pub const V3D_FDBGR: u32 = 0x00F0C;
/// V3D FEP Internal Stall Input Signals
pub const V3D_FDBGS: u32 = 0x00F10;
/// V3D Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C)
pub const V3D_ERRSTAT: u32 = 0x00F20;

// V3D Identity Registers
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_IDENT0: V3D Identification 0 (V3D Block Identity) Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_IDENT0: V3D ID String (Reads As "V3D") READ
pub const IDSTR: u32 = 0x00FFFFFF;
/// V3D_IDENT0: V3D Technology Version (Reads Technology Version = 2) READ
pub const TVER: u32 = 0xFF000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_IDENT1: V3D Identification 1 (V3D Configuration A) Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_IDENT1: V3D Revision READ
pub const REVR: u32 = 0x0000000F;
/// V3D_IDENT1: Number Of Slices READ
pub const NSLC: u32 = 0x000000F0;
/// V3D_IDENT1: Number Of QPUs Per Slice READ
pub const QUPS: u32 = 0x00000F00;
/// V3D_IDENT1: Number Of TMUs Per Slice READ
pub const TUPS: u32 = 0x0000F000;
/// V3D_IDENT1: Number Of Semaphores READ
pub const NSEM: u32 = 0x00FF0000;
/// V3D_IDENT1: HDR Support (0 = Not Supported, 1 = Supported) READ
pub const HDRT: u32 = 0x0F000000;
/// V3D_IDENT1: VPM Memory Size (Multiples Of 1K, 0 => 16K) READ
pub const VPMSZ: u32 = 0xF0000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_IDENT2: V3D Identification 2 (V3D Configuration B) Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_IDENT2: VRI Memory Size (0 = Half Size, 1 = Full Size) READ
pub const VRISZ: u32 = 0x0000000F;
/// V3D_IDENT2: Tile Buffer Size (0 = Quarter Size, 1 = Half Size, 2 = Full Size
/// (32x32msm)) READ
pub const TLBSZ: u32 = 0x000000F0;
/// V3D_IDENT2: Tile Buffer Double-Buffer Mode Support (0 = Not Supported, 1 =
/// Supported) READ
pub const TLBDB: u32 = 0x00000F00;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Miscellaneous Registers
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SCRATCH: V3D Scratch Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SCRATCH: Scratch Register (Read/Write Registers For General Purposes)
/// READ/WRITE
pub const SCRATCH: u32 = 0xFFFFFFFF;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Cache Control Registers
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_L2CACTL: V3D L2 Cache Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_L2CACTL: L2 Cache Enable (Reads State Of Cache Enable Bit, Write To
/// Enable The L2 Cache) READ/WRITE
pub const L2CENA: u32 = 0x00000001;
/// V3D_L2CACTL: L2 Cache Disable (Write To Disable The L2 Cache) WRITE
pub const L2CDIS: u32 = 0x00000002;
/// V3D_L2CACTL: L2 Cache Clear (Write To Clear The L2 Cache) WRITE
pub const L2CCLR: u32 = 0x00000004;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SLCACTL: V3D Slices Cache Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SLCACTL: Instruction Cache Clear Bits (Write To Clear Instruction Cache)
/// WRITE
pub const ICCS0_TO_ICCS3: u32 = 0x0000000F;
/// V3D_SLCACTL: Uniforms Cache Clear Bits (Write To Clear Uniforms Cache) WRITE
pub const UCCS0_TO_UCCS3: u32 = 0x00000F00;
/// V3D_SLCACTL: TMU0 Cache Clear Bits (Write To Clear TMU0 Cache) WRITE
pub const T0CCS0_TO_T0CCS3: u32 = 0x000F0000;
/// V3D_SLCACTL: TMU1 Cache Clear Bits (Write To Clear TMU1 Cache) WRITE
pub const T1CCS0_TO_T1CCS3: u32 = 0x0F000000;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Pipeline Interrupt Control
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_INTCTL: V3D Interrupt Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_INTCTL: Render Mode Frame Done Interrupt Status (Set When All Tiles Of
/// The Frame Have Been Written To Memory) READ/WRITE
pub const INT_FRDONE: u32 = 0x00000001;
/// V3D_INTCTL: Binning Mode Flush Done Interrupt Status (Set When Binning Is
/// Complete With All Tile Lists Flushed To Memory) READ/WRITE
pub const INT_FLDONE: u32 = 0x00000002;
/// V3D_INTCTL: Binner Out Of Memory Interrupt Status (Set While The Binner
/// Needs More Memory To Complete) READ/WRITE
pub const INT_OUTOMEM: u32 = 0x00000004;
/// V3D_INTCTL: Binner Used Overspill Memory Interrupt Status (Set When The
/// Binner Starts Using The (Valid) Overspill Memory Buffer) READ/WRITE
pub const INT_SPILLUSE: u32 = 0x00000008;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_INTENA: V3D Interrupt Enables Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_INTENA: Render Mode Frame Done Interrupt Enable (Set When The INT_FRDONE
/// Interrupt Is Set) READ/WRITE
pub const EI_FRDONE: u32 = 0x00000001;
/// V3D_INTENA: Binning Mode Flush Done Interrupt Enable (Set When The
/// INT_FLDONE Interrupt Is Set) READ/WRITE
pub const EI_FLDONE: u32 = 0x00000002;
/// V3D_INTENA: Binner Out Of Memory Interrupt Enable (Set When The INT_OUTOMEM
/// Interrupt Is Set) READ/WRITE
pub const EI_OUTOMEM: u32 = 0x00000004;
/// V3D_INTENA: Binner Used Overspill Memory Interrupt Enable (Set When The
/// INT_SPILLUSE Interrupt Is Set) READ/WRITE
pub const EI_SPILLUSE: u32 = 0x00000008;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_INTDIS: V3D Interrupt Disables Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_INTDIS: Render Mode Frame Done Interrupt Disable (Set When The
/// INT_FRDONE Interrupt Is Set) READ/WRITE
pub const DI_FRDONE: u32 = 0x00000001;
/// V3D_INTDIS: Binning Mode Flush Done Interrupt Disable (Set When The
/// INT_FLDONE Interrupt Is Set) READ/WRITE
pub const DI_FLDONE: u32 = 0x00000002;
/// V3D_INTDIS: Binner Out Of Memory Interrupt Disable (Set When The INT_OUTOMEM
/// Interrupt Is Set) READ/WRITE
pub const DI_OUTOMEM: u32 = 0x00000004;
/// V3D_INTDIS: Binner Used Overspill Memory Interrupt Disable (Set When The
/// INT_SPILLUSE Interrupt Is Set) READ/WRITE
pub const DI_SPILLUSE: u32 = 0x00000008;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Control List Executor Registers (Per Thread)
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnCS: V3D Control List Executor Thread n Control & Status Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnCS: Control Thread Mode (Binning Mode Thread Only) READ
pub const CTMODE: u32 = 0x00000001;
/// V3D_CTnCS: Control Thread Error (Set When Stopped With An Error, Cleared On
/// Restarting) READ
pub const CTERR: u32 = 0x00000008;
/// V3D_CTnCS: Control Thread Sub-Mode READ/WRITE
pub const CTSUBS: u32 = 0x00000010;
/// V3D_CTnCS: Control Thread Run READ/WRITE
pub const CTRUN: u32 = 0x00000020;
/// V3D_CTnCS: Return Stack Depth (Number Of Levels Of List Nesting) READ
pub const CTRTSD: u32 = 0x00000300;
/// V3D_CTnCS: Counting Semaphore (Current State Of The Counting Semaphore For
/// This Thread) READ
pub const CTSEMA: u32 = 0x00007000;
/// V3D_CTnCS: Reset Bit (Writing 1 Stops The Control Thread & Resets All Bits
/// In This Register) WRITE
pub const CTRSTA: u32 = 0x00008000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnEA: V3D Control List Executor Thread n End Address Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnEA: Control List End Address (Set To The Byte Address After The Last
/// Record In The Control List) READ/WRITE
pub const CTLEA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnCA: V3D Control List Executor Thread n Current Address Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnCA: Control List Current Address (Points To The Address Of The
/// Current Record In The Control List) READ/WRITE
pub const CTLCA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnRA0: V3D Control List Executor Thread n Return Address Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnRA0: Control List Return Address 0 (Address On Return Address Stack)
/// READ
pub const CTLRA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnLC: V3D Control List Executor Thread n List Counter Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnLC: Sub-list Counter (Count Of Return Commands Encountered)
/// READ/WRITE
pub const CTLSLCS: u32 = 0x0000FFFF;
/// V3D_CTnLC: Major List Counter (Count Of Flush Commands Encountered)
/// READ/WRITE
pub const CTLLCM: u32 = 0xFFFF0000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_CTnPC: V3D Control List Executor Thread n Primitive List Counter Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_CTnPC: Primitive List Counter (Count Of Primitives Remaining Whilst
/// Processing A Primitive List) READ
pub const CTLPC: u32 = 0xFFFFFFFF;

//;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Pipeline Registers
//;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_PCS: V3D Pipeline Control & Status Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_PCS: Binning Mode Active (Set While Binning Pipeline Is In Use) READ
pub const BMACTIVE: u32 = 0x00000001;
/// V3D_PCS: Binning Mode Busy (Set While Any Binning Operations Are Actually In
/// Progress) READ
pub const BMBUSY: u32 = 0x00000002;
/// V3D_PCS: Rendering Mode Active (Set While Rendering Pipeline Is In Use) READ
pub const RMACTIVE: u32 = 0x00000004;
/// V3D_PCS: Rendering Mode Busy (Set While Any Rendering Operations Are
/// Actually In Progress) READ
pub const RMBUSY: u32 = 0x00000008;
/// V3D_PCS: Binning Mode Out Of Memory (Set When PTB Runs Out Of Binning Memory
/// While Binning) READ
pub const BMOOM: u32 = 0x00000100;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BFC: V3D Binning Mode Flush Count Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BFC: Flush Count (Count Increments In Binning Mode Once PTB Has Flushed
/// All Tile Lists To Mem & PTB Has Finished With Tile State Data Array)
/// READ/WRITE
pub const BMFCT: u32 = 0x000000FF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_RFC: V3D Rendering Mode Frame Count Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_RFC: Frame Count (Count Increments In Rendering Mode When Last Tile
/// Store Operation Of Frame Completes, The Tile Has Fully Written Out To Mem)
/// READ/WRITE
pub const RMFCT: u32 = 0x000000FF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BPCA: V3D Current Address Of Binning Memory Pool Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BPCA: Current Pool Address (The Address Of The Current Allocation
/// Pointer In The Binning Memory Pool) READ
pub const BMPCA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BPCS: V3D Remaining Size Of Binning Memory Pool Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BPCS: Size Of Pool Remaining (The Number Of Bytes Remaining In The
/// Binning Memory Pool) READ
pub const BMPRS: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BPOA: V3D Address Of Overspill Binning Memory Block Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BPOA: Address Of Overspill Memory Block For Binning (Address Of
/// Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out)
/// READ/WRITE
pub const BMPOA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BPOS: V3D Size Of Overspill Binning Memory Block Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BPOS: Size Of Overspill Memory Block For Binning (Number Of Bytes Of
/// Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out)
/// READ/WRITE
pub const BMPOS: u32 = 0xFFFFFFFF;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D QPU Scheduler Registers
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SQRSV0: V3D Reserve QPUs 0-7 Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SQRSV0: Reservation Settings For QPU 0 READ/WRITE
pub const QPURSV0: u32 = 0x0000000F;
/// V3D_SQRSV0: Reservation Settings For QPU 1 READ/WRITE
pub const QPURSV1: u32 = 0x000000F0;
/// V3D_SQRSV0: Reservation Settings For QPU 2 READ/WRITE
pub const QPURSV2: u32 = 0x00000F00;
/// V3D_SQRSV0: Reservation Settings For QPU 3 READ/WRITE
pub const QPURSV3: u32 = 0x0000F000;
/// V3D_SQRSV0: Reservation Settings For QPU 4 READ/WRITE
pub const QPURSV4: u32 = 0x000F0000;
/// V3D_SQRSV0: Reservation Settings For QPU 5 READ/WRITE
pub const QPURSV5: u32 = 0x00F00000;
/// V3D_SQRSV0: Reservation Settings For QPU 6 READ/WRITE
pub const QPURSV6: u32 = 0x0F000000;
/// V3D_SQRSV0: Reservation Settings For QPU 7 READ/WRITE
pub const QPURSV7: u32 = 0xF0000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SQRSV1: V3D Reserve QPUs 8-15 Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SQRSV1: Reservation Settings For QPU 8 READ/WRITE
pub const QPURSV8: u32 = 0x0000000F;
/// V3D_SQRSV1: Reservation Settings For QPU 9 READ/WRITE
pub const QPURSV9: u32 = 0x000000F0;
/// V3D_SQRSV1: Reservation Settings For QPU 10 READ/WRITE
pub const QPURSV10: u32 = 0x00000F00;
/// V3D_SQRSV1: Reservation Settings For QPU 11 READ/WRITE
pub const QPURSV11: u32 = 0x0000F000;
/// V3D_SQRSV1: Reservation Settings For QPU 12 READ/WRITE
pub const QPURSV12: u32 = 0x000F0000;
/// V3D_SQRSV1: Reservation Settings For QPU 13 READ/WRITE
pub const QPURSV13: u32 = 0x00F00000;
/// V3D_SQRSV1: Reservation Settings For QPU 14 READ/WRITE
pub const QPURSV14: u32 = 0x0F000000;
/// V3D_SQRSV1: Reservation Settings For QPU 15 READ/WRITE
pub const QPURSV15: u32 = 0xF0000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SQCNTL: V3D QPU Scheduler Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SQCNTL: Vertex Shader Scheduling Bypass Limit READ/WRITE
pub const VSRBL: u32 = 0x00000003;
/// V3D_SQCNTL: Coordinate Shader Scheduling Bypass Limit READ/WRITE
pub const CSRBL: u32 = 0x0000000C;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SRQPC: V3D QPU User Program Request Program Address Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SRQPC: Program Address (Writing This Register Queues A Request To Run A
/// Program Starting At The Given Address) WRITE
pub const QPURQPC: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SRQUA: V3D QPU User Program Request Uniforms Address Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SRQUA: Uniforms Address (Contains The Address Of The Uniforms Stream For
/// The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE
pub const QPURQUA: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SRQUL: V3D QPU User Program Request Uniforms Length Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SRQUL: Uniforms Length (Contains The Max Length Of The Uniforms Stream
/// For The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE
pub const QPURQUL: u32 = 0x00000FFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_SRQCS: V3D QPU User Program Request Control & Status Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_SRQCS: Queue Length (Contains The Number Of Program Requests Currently
/// Queued) READ/WRITE
pub const QPURQL: u32 = 0x0000003F;
/// V3D_SRQCS: Queue Error (Set When A Request Has Been Made When The Queue Is
/// Full) READ/WRITE
pub const QPURQERR: u32 = 0x00000080;
/// V3D_SRQCS: Count Of User Program Requests Made (Contains The Total Number Of
/// User Program Requests Made, Modulo 256) READ/WRITE
pub const QPURQCM: u32 = 0x0000FF00;
/// V3D_SRQCS: Count Of User Programs Completed (Contains The Total Number Of
/// User Programs That Have Run & Completed, Modulo 256) READ/WRITE
pub const QPURQCC: u32 = 0x00FF0000;

//;;;;;;;;;;;;;;;;;;;
// V3D VPM Registers
//;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_VPACNTL: V3D VPM Allocator Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_VPACNTL: Rendering VPM Allocation Limit (Limits The Amount Of VPM Memory
/// Allocated To Rendering Mode) READ/WRITE
pub const VPARALIM: u32 = 0x00000007;
/// V3D_VPACNTL: Binning VPM Allocation Limit (Limits The Amount Of VPM Memory
/// Allocated To Binning Mode) READ/WRITE
pub const VPABALIM: u32 = 0x00000038;
/// V3D_VPACNTL: Rendering VPM Allocation Timeout (Sets A Timeout For Raising
/// The Priority Of Rendering Mode Allocation Requests) READ/WRITE
pub const VPARATO: u32 = 0x000001C0;
/// V3D_VPACNTL: Binning VPM Allocation Timeout (Sets A Timeout For Raising The
/// Priority Of Binning Mode Allocation Requests) READ/WRITE
pub const VPABATO: u32 = 0x00000E00;
/// V3D_VPACNTL: Enable VPM Allocation Limits (Enables VPM Memory Allocation
/// Limiting Using VPARALIM & VPABALIM) READ/WRITE
pub const VPALIMEN: u32 = 0x00001000;
/// V3D_VPACNTL: Enable VPM Allocation Timeout (Enables VPM Memory Allocation
/// Timeout Using VPARATO & VPABATO) READ/WRITE
pub const VPATOEN: u32 = 0x00002000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_VPMBASE: V3D VPM Base (User) Memory Reservation Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_VPMBASE: VPM Memory Reserved For User Programs (Contains Amount Of VPM
/// Mem Reserved For All User Programs, In Multiples Of 256 Bytes) READ/WRITE
pub const VPMURSV: u32 = 0x0000001F;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D QPU Interrupt Control
//;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_DBQITE: V3D QPU Interrupt Enables Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_DBQITE: QPU Interrupt Enable bits (Set Bit To Allow QPU To Generate An
/// Interrupt) READ/WRITE
pub const IE_QPU0_TO_IE_QPU15: u32 = 0x0000FFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_DBQITC: V3D QPU Interrupt Control Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_DBQITC: QPU Interrupt Control Bits (Reads When Interrupt Is Latched,
/// Write To Clear Interrupt) READ/WRITE
pub const IC_QPU0_TO_IC_QPU15: u32 = 0x0000FFFF;

//;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Performance Counters
//;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D Sources For Performance Counters
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// FEP Valid Primitives That Result In No Rendered Pixels, For All Rendered
/// Tiles
pub const COUNT_ID_0: u32 = 0;
/// FEP Valid Primitives For All Rendered Tiles (Primitives May Be Counted In
/// More Than One Tile)
pub const COUNT_ID_1: u32 = 1;
/// FEP Early-Z/Near/Far Clipped Quads
pub const COUNT_ID_2: u32 = 2;
/// FEP Valid Quads
pub const COUNT_ID_3: u32 = 3;
/// TLB Quads With No Pixels Passing The Stencil Test
pub const COUNT_ID_4: u32 = 4;
/// TLB Quads With No Pixels Passing The Z & Stencil Tests
pub const COUNT_ID_5: u32 = 5;
/// TLB Quads With Any Pixels Passing The Z & Stencil Tests
pub const COUNT_ID_6: u32 = 6;
/// TLB Quads With All Pixels Having Zero Coverage
pub const COUNT_ID_7: u32 = 7;
/// TLB Quads With Any Pixels Having Non-Zero Coverage
pub const COUNT_ID_8: u32 = 8;
/// TLB Quads With Valid Pixels Written To Color Buffer
pub const COUNT_ID_9: u32 = 9;
/// PTB Primitives Discarded By Being Outside The Viewport
pub const COUNT_ID_10: u32 = 10;
/// PTB Primitives That Need Clipping
pub const COUNT_ID_11: u32 = 11;
/// PSE Primitives That Are Discarded Because They Are Reversed
pub const COUNT_ID_12: u32 = 12;
/// QPU Total Idle Clock Cycles For All QPUs
pub const COUNT_ID_13: u32 = 13;
/// QPU Total Clock Cycles For All QPUs Doing Vertex/Coordinate Shading
pub const COUNT_ID_14: u32 = 14;
/// QPU Total Clock Cycles For All QPUs Doing Fragment Shading
pub const COUNT_ID_15: u32 = 15;
/// QPU Total Clock Cycles For All QPUs Executing Valid Instructions
pub const COUNT_ID_16: u32 = 16;
/// QPU Total Clock Cycles For All QPUs Stalled Waiting For TMUs
pub const COUNT_ID_17: u32 = 17;
/// QPU Total Clock Cycles For All QPUs Stalled Waiting For Scoreboard
pub const COUNT_ID_18: u32 = 18;
/// QPU Total Clock Cycles For All QPUs Stalled Waiting For Varyings
pub const COUNT_ID_19: u32 = 19;
/// QPU Total Instruction Cache Hits For All Slices
pub const COUNT_ID_20: u32 = 20;
/// QPU Total Instruction Cache Misses For All Slices
pub const COUNT_ID_21: u32 = 21;
/// QPU Total Uniforms Cache Hits For All Slices
pub const COUNT_ID_22: u32 = 22;
/// QPU Total Uniforms Cache Misses For All Slices
pub const COUNT_ID_23: u32 = 23;
/// TMU Total Texture Quads Processed
pub const COUNT_ID_24: u32 = 24;
/// TMU Total Texture Cache Misses (Number Of Fetches From Memory/L2 Cache)
pub const COUNT_ID_25: u32 = 25;
/// VPM Total Clock Cycles VDW Is Stalled Waiting For VPM Access
pub const COUNT_ID_26: u32 = 26;
/// VPM Total Clock Cycles VCD Is Stalled Waiting For VPM Access
pub const COUNT_ID_27: u32 = 27;
/// L2C Total Level 2 Cache Hits
pub const COUNT_ID_28: u32 = 28;
/// L2C Total Level 2 Cache Misses
pub const COUNT_ID_29: u32 = 29;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_PCTRC: V3D Performance Counter Clear Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_PCTRC: Performance Counter Clear Bits (Write To Clear The Performance
/// Counter) WRITE
pub const CTCLR0_CTCLR15: u32 = 0x0000FFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_PCTRE: V3D Performance Counter Enables Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_PCTRE: Performance Counter Enable Bits (0 = Counter Disabled, 1 =
/// Performance Counter Enabled To Count) READ/WRITE
pub const CTEN0_CTEN15: u32 = 0x0000FFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_PCTRn: V3D Performance Counter Count n Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_PCTRn: Performance Count (Count Value) READ/WRITE
pub const PCTR: u32 = 0xFFFFFFFF;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_PCTRSn: V3D Performance Counter Mapping n Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_PCTRSn: Performance Counter Device ID READ/WRITE
pub const PCTRS: u32 = 0x0000001F;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// V3D Error & Diagnostic Registers
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_BXCF: V3D Binner Debug Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_BXCF: Disable Forwarding In State Cache READ/WRITE
pub const FWDDISA: u32 = 0x00000001;
/// V3D_BXCF: Disable Clipping READ/WRITE
pub const CLIPDISA: u32 = 0x00000002;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_DBGE: V3D PSE Error Signals Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_DBGE: Error A Reading VPM READ
pub const VR1_A: u32 = 0x00000002;
/// V3D_DBGE: Error B Reading VPM READ
pub const VR1_B: u32 = 0x00000004;
/// V3D_DBGE: Error Mulip 0 READ
pub const MULIP0: u32 = 0x00010000;
/// V3D_DBGE: Error Mulip 1 READ
pub const MULIP1: u32 = 0x00020000;
/// V3D_DBGE: Error Mulip 2 READ
pub const MULIP2: u32 = 0x00040000;
/// V3D_DBGE: Error IPD2 Valid READ
pub const IPD2_VALID: u32 = 0x00080000;
/// V3D_DBGE: Error IPD2 FPD Used READ
pub const IPD2_FPDUSED: u32 = 0x00100000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_FDBGO: V3D FEP Overrun Error Signals Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_FDBGO: Not An Error READ
pub const WCOEFF_FIFO_FULL: u32 = 0x00000002;
/// V3D_FDBGO: Not An Error READ
pub const XYRELZ_FIFO_FULL: u32 = 0x00000004;
/// V3D_FDBGO: Error READ
pub const QBFR_FIFO_ORUN: u32 = 0x00000008;
/// V3D_FDBGO: Error READ
pub const QBSZ_FIFO_ORUN: u32 = 0x00000010;
/// V3D_FDBGO: Error READ
pub const XYFO_FIFO_ORUN: u32 = 0x00000020;
/// V3D_FDBGO: Error READ
pub const FIXZ_ORUN: u32 = 0x00000040;
/// V3D_FDBGO: Error READ
pub const XYRELO_FIFO_ORUN: u32 = 0x00000080;
/// V3D_FDBGO: Error READ
pub const XYRELW_FIFO_ORUN: u32 = 0x00000400;
/// V3D_FDBGO: Not An Error
pub const ZCOEFF_FIFO_FULL: u32 = 0x00000800;
/// V3D_FDBGO: Error READ
pub const REFXY_FIFO_ORUN: u32 = 0x00001000;
/// V3D_FDBGO: Error READ
pub const DEPTHO_FIFO_ORUN: u32 = 0x00002000;
/// V3D_FDBGO: Error READ
pub const DEPTHO_ORUN: u32 = 0x00004000;
/// V3D_FDBGO: Error READ
pub const EZVAL_FIFO_ORUN: u32 = 0x00008000;
/// V3D_FDBGO: Error READ
pub const EZREQ_FIFO_ORUN: u32 = 0x00020000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_FDBGB: V3D FEP Interface Ready & Stall Signals, FEP Busy Signals Register
// Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_FDBGB: Stall READ
pub const EDGES_STALL: u32 = 0x00000001;
/// V3D_FDBGB: Ready READ
pub const EDGES_READY: u32 = 0x00000002;
/// V3D_FDBGB: READ
pub const EDGES_ISCTRL: u32 = 0x00000004;
/// V3D_FDBGB: READ
pub const EDGES_CTRLID: u32 = 0x00000038;
/// V3D_FDBGB: Stall READ
pub const ZRWPE_STALL: u32 = 0x00000040;
/// V3D_FDBGB: Ready READ
pub const ZRWPE_READY: u32 = 0x00000080;
/// V3D_FDBGB: Ready READ
pub const EZ_DATA_READY: u32 = 0x00800000;
/// V3D_FDBGB: Ready READ
pub const EZ_XY_READY: u32 = 0x02000000;
/// V3D_FDBGB: Busy READ
pub const RAST_BUSY: u32 = 0x04000000;
/// V3D_FDBGB: Ready READ
pub const QXYF_FIFO_OP_READY: u32 = 0x08000000;
/// V3D_FDBGB: Ready READ
pub const XYFO_FIFO_OP_READY: u32 = 0x10000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_FDBGR: V3D FEP Internal Ready Signals Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_FDBGR: Ready READ
pub const QXYF_FIFO_READY: u32 = 0x00000001;
/// V3D_FDBGR: Ready READ
pub const EZREQ_FIFO_READY: u32 = 0x00000002;
/// V3D_FDBGR: Ready READ
pub const EZVAL_FIFO_READY: u32 = 0x00000004;
/// V3D_FDBGR: Ready READ
pub const DEPTHO_FIFO_READY: u32 = 0x00000008;
/// V3D_FDBGR: Ready READ
pub const REFXY_FIFO_READY: u32 = 0x00000010;
/// V3D_FDBGR: Ready READ
pub const ZCOEFF_FIFO_READY: u32 = 0x00000020;
/// V3D_FDBGR: Ready READ
pub const XYRELW_FIFO_READY: u32 = 0x00000040;
/// V3D_FDBGR: Ready READ
pub const WCOEFF_FIFO_READY: u32 = 0x00000080;
/// V3D_FDBGR: Ready READ
pub const XYRELO_FIFO_READY: u32 = 0x00000800;
/// V3D_FDBGR: Ready READ
pub const ZO_FIFO_READY: u32 = 0x00002000;
/// V3D_FDBGR: Ready READ
pub const XYFO_FIFO_READY: u32 = 0x00004000;
/// V3D_FDBGR: Ready READ
pub const RAST_READY: u32 = 0x00010000;
/// V3D_FDBGR: Last READ
pub const RAST_LAST: u32 = 0x00020000;
/// V3D_FDBGR: Ready READ
pub const DEPTHO_READY: u32 = 0x00040000;
/// V3D_FDBGR: Ready READ
pub const EZLIM_READY: u32 = 0x00080000;
/// V3D_FDBGR: Ready READ
pub const XYNRM_READY: u32 = 0x00100000;
/// V3D_FDBGR: Last READ
pub const XYNRM_LAST: u32 = 0x00200000;
/// V3D_FDBGR: Ready READ
pub const XYRELZ_FIFO_READY: u32 = 0x00400000;
/// V3D_FDBGR: Last READ
pub const XYRELZ_FIFO_LAST: u32 = 0x00800000;
/// V3D_FDBGR: Ready READ
pub const INTERPZ_READY: u32 = 0x01000000;
/// V3D_FDBGR: Ready READ
pub const INTERPRW_READY: u32 = 0x08000000;
/// V3D_FDBGR: Ready READ
pub const RECIPW_READY: u32 = 0x10000000;
/// V3D_FDBGR: Ready READ
pub const FIXZ_READY: u32 = 0x40000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_FDBGS: V3D FEP Internal Stall Input Signals Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_FDBGS: Stall READ
pub const EZTEST_IP_QSTALL: u32 = 0x00000001;
/// V3D_FDBGS: Stall READ
pub const EZTEST_IP_PRSTALL: u32 = 0x00000002;
/// V3D_FDBGS: Stall READ
pub const EZTEST_IP_VLFSTALL: u32 = 0x00000004;
/// V3D_FDBGS: Stall READ
pub const EZTEST_STALL: u32 = 0x00000008;
/// V3D_FDBGS: Valid READ
pub const EZTEST_VLF_OKNOVALID: u32 = 0x00000010;
/// V3D_FDBGS: Ready READ
pub const EZTEST_QREADY: u32 = 0x00000020;
/// V3D_FDBGS: READ
pub const EZTEST_ANYQF: u32 = 0x00000040;
/// V3D_FDBGS: Valid READ
pub const EZTEST_ANYQVALID: u32 = 0x00000080;
/// V3D_FDBGS: Valid READ
pub const QXYF_FIFO_OP1_VALID: u32 = 0x00000100;
/// V3D_FDBGR: Last READ
pub const QXYF_FIFO_OP1_LAST: u32 = 0x00000200;
/// V3D_FDBGR: Dummy READ
pub const QXYF_FIFO_OP1_DUMMY: u32 = 0x00000400;
/// V3D_FDBGR: Last READ
pub const QXYF_FIFO_OP_LAST: u32 = 0x00000800;
/// V3D_FDBGS: Valid READ
pub const QXYF_FIFO_OP_VALID: u32 = 0x00001000;
/// V3D_FDBGS: Valid READ
pub const EZREQ_FIFO_OP_VALID: u32 = 0x00002000;
/// V3D_FDBGS: Stall READ
pub const XYNRM_IP_STALL: u32 = 0x00004000;
/// V3D_FDBGS: Stall READ
pub const EZLIM_IP_STALL: u32 = 0x00008000;
/// V3D_FDBGS: Stall READ
pub const DEPTHO_FIFO_IP_STALL: u32 = 0x00010000;
/// V3D_FDBGS: Stall READ
pub const INTERPZ_IP_STALL: u32 = 0x00020000;
/// V3D_FDBGS: Stall READ
pub const XYRELZ_FIFO_IP_STALL: u32 = 0x00040000;
/// V3D_FDBGS: Stall READ
pub const INTERPW_IP_STALL: u32 = 0x00400000;
/// V3D_FDBGS: Stall READ
pub const RECIPW_IP_STALL: u32 = 0x02000000;
/// V3D_FDBGS: Stall READ
pub const ZO_FIFO_IP_STALL: u32 = 0x10000000;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D_ERRSTAT: V3D Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C)
// Register Description
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

/// V3D_ERRSTAT: VPM Allocator Error - Allocating Base While Busy READ
pub const VPAEABB: u32 = 0x00000001;
/// V3D_ERRSTAT: VPM Allocator Error - Request Too Big READ
pub const VPAERGS: u32 = 0x00000002;
/// V3D_ERRSTAT: VPM Allocator Error - Binner Request Greater Than Limit READ
pub const VPAEBRGL: u32 = 0x00000004;
/// V3D_ERRSTAT: VPM Allocator Error - Renderer Request Greater Than Limit READ
pub const VPAERRGL: u32 = 0x00000008;
/// V3D_ERRSTAT: VPM Error - Write Range READ
pub const VPMEWR: u32 = 0x00000010;
/// V3D_ERRSTAT: VPM Error - Read Range READ
pub const VPMERR: u32 = 0x00000020;
/// V3D_ERRSTAT: VPM Error - Read Non-Allocated READ
pub const VPMERNA: u32 = 0x00000040;
/// V3D_ERRSTAT: VPM Error - Write Non-Allocated READ
pub const VPMEWNA: u32 = 0x00000080;
/// V3D_ERRSTAT: VPM Error - Free Non-Allocated READ
pub const VPMEFNA: u32 = 0x00000100;
/// V3D_ERRSTAT: VPM Error - Allocated Size Error READ
pub const VPMEAS: u32 = 0x00000200;
/// V3D_ERRSTAT: VDW Error - Address Overflows READ
pub const VDWE: u32 = 0x00000400;
/// V3D_ERRSTAT: VCD Error - FIFO Pointers Out Of Sync READ
pub const VCDE: u32 = 0x00000800;
/// V3D_ERRSTAT: VCD Idle READ
pub const VCDI: u32 = 0x00001000;
/// V3D_ERRSTAT: VCM Error (Renderer) READ
pub const VCMRE: u32 = 0x00002000;
/// V3D_ERRSTAT: VCM Error (Binner) READ
pub const VCMBE: u32 = 0x00004000;
/// V3D_ERRSTAT: L2C AXI Receive Fifo Overrun Error READ
pub const L2CARE: u32 = 0x00008000;
