#![allow(unused)]

// Raspberry Pi VideoCoreIV
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// V3D Register Address Map
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

macro_rules! volatile_registers {
    ($($(#[$attr:meta])* $name:ident = $value:literal);* $(;)?) => {
        $($(#[$attr])*
        pub static mut $name: *mut u32 = (crate::PERIPHERAL_BASE.addr + V3D_BASE + $value) as *mut u32;
        )*
    };
}

#[doc = "V3D Base Address ($20C00000 PHYSICAL, $7EC00000 BUS)"]
pub const V3D_BASE: usize = 0xC00000;

volatile_registers! {
    #[doc="V3D Identification 0 (V3D Block Identity)"]
    V3D_IDENT0 = 0x00000;
    #[doc="V3D Identification 1 (V3D Configuration A)"]
    V3D_IDENT1 = 0x00004;
    #[doc="V3D Identification 2 (V3D Configuration B)"]
    V3D_IDENT2 = 0x00008;
    #[doc="V3D Identification 3 (V3D Configuration C)"]
    V3D_IDENT3 = 0x0000C;
    #[doc="V3D Scratch Register"]
    V3D_SCRATCH = 0x00010;
    #[doc="V3D L2 Cache Control"]
    V3D_L2CACTL = 0x00020;
    #[doc="V3D Slices Cache Control"]
    V3D_SLCACTL = 0x00024;
    #[doc="V3D Interrupt Control"]
    V3D_INTCTL = 0x00030;
    #[doc="V3D Interrupt Enables"]
    V3D_INTENA = 0x00034;
    #[doc="V3D Interrupt Disables"]
    V3D_INTDIS = 0x00038;
    #[doc="V3D Control List Executor Thread 0 Control & Status"]
    V3D_CT0CS = 0x00100;
    #[doc="V3D Control List Executor Thread 1 Control & Status"]
    V3D_CT1CS = 0x00104;
    #[doc="V3D Control List Executor Thread 0 End Address"]
    V3D_CT0EA = 0x00108;
    #[doc="V3D Control List Executor Thread 1 End Address"]
    V3D_CT1EA = 0x0010C;
    #[doc="V3D Control List Executor Thread 0 Current Address"]
    V3D_CT0CA = 0x00110;
    #[doc="V3D Control List Executor Thread 1 Current Address"]
    V3D_CT1CA = 0x00114;
    #[doc="V3D Control List Executor Thread 0 Return Address"]
    V3D_CT0RA0 = 0x00118;
    #[doc="V3D Control List Executor Thread 1 Return Address"]
    V3D_CT1RA0 = 0x0011C;
    #[doc="V3D Control List Executor Thread 0 List Counter"]
    V3D_CT0LC = 0x00120;
    #[doc="V3D Control List Executor Thread 1 List Counter"]
    V3D_CT1LC = 0x00124;
    #[doc="V3D Control List Executor Thread 0 Primitive List Counter"]
    V3D_CT0PC = 0x00128;
    #[doc="V3D Control List Executor Thread 1 Primitive List Counter"]
    V3D_CT1PC = 0x0012C;
    #[doc="V3D Pipeline Control & Status"]
    V3D_PCS = 0x00130;
    #[doc="V3D Binning Mode Flush Count"]
    V3D_BFC = 0x00134;
    #[doc="V3D Rendering Mode Frame Count"]
    V3D_RFC = 0x00138;
    #[doc="V3D Current Address Of Binning Memory Pool"]
    V3D_BPCA = 0x00300;
    #[doc="V3D Remaining Size Of Binning Memory Pool"]
    V3D_BPCS = 0x00304;
    #[doc="V3D Address Of Overspill Binning Memory Block"]
    V3D_BPOA = 0x00308;
    #[doc="V3D Size Of Overspill Binning Memory Block"]
    V3D_BPOS = 0x0030C;
    #[doc="V3D Binner Debug"]
    V3D_BXCF = 0x00310;
    #[doc="V3D Reserve QPUs 0-7"]
    V3D_SQRSV0 = 0x00410;
    #[doc="V3D Reserve QPUs 8-15"]
    V3D_SQRSV1 = 0x00414;
    #[doc="V3D QPU Scheduler Control"]
    V3D_SQCNTL = 0x00418;
    #[doc="V3D QPU Scheduler State"]
    V3D_SQCSTAT = 0x0041C;
    #[doc="V3D QPU User Program Request Program Address"]
    V3D_SRQPC = 0x00430;
    #[doc="V3D QPU User Program Request Uniforms Address"]
    V3D_SRQUA = 0x00434;
    #[doc="V3D QPU User Program Request Uniforms Length"]
    V3D_SRQUL = 0x00438;
    #[doc="V3D QPU User Program Request Control & Status"]
    V3D_SRQCS = 0x0043C;
    #[doc="V3D VPM Allocator Control"]
    V3D_VPACNTL = 0x00500;
    #[doc="V3D VPM Base (User) Memory Reservation"]
    V3D_VPMBASE = 0x00504;
    #[doc="V3D Performance Counter Clear"]
    V3D_PCTRC = 0x00670;
    #[doc="V3D Performance Counter Enables"]
    V3D_PCTRE = 0x00674;
    #[doc="V3D Performance Counter Count 0"]
    V3D_PCTR0 = 0x00680;
    #[doc="V3D Performance Counter Mapping 0"]
    V3D_PCTRS0 = 0x00684;
    #[doc="V3D Performance Counter Count 1"]
    V3D_PCTR1 = 0x00688;
    #[doc="V3D Performance Counter Mapping 1"]
    V3D_PCTRS1 = 0x0068C;
    #[doc="V3D Performance Counter Count 2"]
    V3D_PCTR2 = 0x00690;
    #[doc="V3D Performance Counter Mapping 2"]
    V3D_PCTRS2 = 0x00694;
    #[doc="V3D Performance Counter Count 3"]
    V3D_PCTR3 = 0x00698;
    #[doc="V3D Performance Counter Mapping 3"]
    V3D_PCTRS3 = 0x0069C;
    #[doc="V3D Performance Counter Count 4"]
    V3D_PCTR4 = 0x006A0;
    #[doc="V3D Performance Counter Mapping 4"]
    V3D_PCTRS4 = 0x006A4;
    #[doc="V3D Performance Counter Count 5"]
    V3D_PCTR5 = 0x006A8;
    #[doc="V3D Performance Counter Mapping 5"]
    V3D_PCTRS5 = 0x006AC;
    #[doc="V3D Performance Counter Count 6"]
    V3D_PCTR6 = 0x006B0;
    #[doc="V3D Performance Counter Mapping 6"]
    V3D_PCTRS6 = 0x006B4;
    #[doc="V3D Performance Counter Count 7"]
    V3D_PCTR7 = 0x006B8;
    #[doc="V3D Performance Counter Mapping 7"]
    V3D_PCTRS7 = 0x006BC;
    #[doc="V3D Performance Counter Count 8"]
    V3D_PCTR8 = 0x006C0;
    #[doc="V3D Performance Counter Mapping 8"]
    V3D_PCTRS8 = 0x006C4;
    #[doc="V3D Performance Counter Count 9"]
    V3D_PCTR9 = 0x006C8;
    #[doc="V3D Performance Counter Mapping 9"]
    V3D_PCTRS9 = 0x006CC;
    #[doc="V3D Performance Counter Count 10"]
    V3D_PCTR10 = 0x006D0;
    #[doc="V3D Performance Counter Mapping 10"]
    V3D_PCTRS10 = 0x006D4;
    #[doc="V3D Performance Counter Count 11"]
    V3D_PCTR11 = 0x006D8;
    #[doc="V3D Performance Counter Mapping 11"]
    V3D_PCTRS11 = 0x006DC;
    #[doc="V3D Performance Counter Count 12"]
    V3D_PCTR12 = 0x006E0;
    #[doc="V3D Performance Counter Mapping 12"]
    V3D_PCTRS12 = 0x006E4;
    #[doc="V3D Performance Counter Count 13"]
    V3D_PCTR13 = 0x006E8;
    #[doc="V3D Performance Counter Mapping 13"]
    V3D_PCTRS13 = 0x006EC;
    #[doc="V3D Performance Counter Count 14"]
    V3D_PCTR14 = 0x006F0;
    #[doc="V3D Performance Counter Mapping 14"]
    V3D_PCTRS14 = 0x006F4;
    #[doc="V3D Performance Counter Count 15"]
    V3D_PCTR15 = 0x006F8;
    #[doc="V3D Performance Counter Mapping 15"]
    V3D_PCTRS15 = 0x006FC;
    #[doc="V3D Configure"]
    V3D_DBCFG = 0x00E00;
    #[doc="V3D S Control & Status"]
    V3D_DBSCS = 0x00E04;
    #[doc="V3D S Configure"]
    V3D_DBSCFG = 0x00E08;
    #[doc="V3D S SR"]
    V3D_DBSSR = 0x00E0C;
    #[doc="V3D SD R0"]
    V3D_DBSDR0 = 0x00E10;
    #[doc="V3D SD R1"]
    V3D_DBSDR1 = 0x00E14;
    #[doc="V3D SD R2"]
    V3D_DBSDR2 = 0x00E18;
    #[doc="V3D SD R3"]
    V3D_DBSDR3 = 0x00E1C;
    #[doc="V3D QPU Run"]
    V3D_DBQRUN = 0x00E20;
    #[doc="V3D QPU Halt"]
    V3D_DBQHLT = 0x00E24;
    #[doc="V3D QPU Step"]
    V3D_DBQSTP = 0x00E28;
    #[doc="V3D QPU Interrupt Enables"]
    V3D_DBQITE = 0x00E2C;
    #[doc="V3D QPU Interrupt Control"]
    V3D_DBQITC = 0x00E30;
    #[doc="V3D QPU GHC"]
    V3D_DBQGHC = 0x00E34;
    #[doc="V3D QPU GHG"]
    V3D_DBQGHG = 0x00E38;
    #[doc="V3D QPU GHH"]
    V3D_DBQGHH = 0x00E3C;
    #[doc="V3D PSE Error Signals"]
    V3D_DBGE = 0x00F00;
    #[doc="V3D FEP Overrun Error Signals"]
    V3D_FDBGO = 0x00F04;
    #[doc="V3D FEP Interface Ready & Stall Signals, FEP Busy Signals"]
    V3D_FDBGB = 0x00F08;
    #[doc="V3D FEP Internal Ready Signals"]
    V3D_FDBGR = 0x00F0C;
    #[doc="V3D FEP Internal Stall Input Signals"]
    V3D_FDBGS = 0x00F10;
    #[doc="V3D Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C)"]
    V3D_ERRSTAT = 0x00F20;

    // V3D Identity Registers
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_IDENT0: V3D Identification 0 (V3D Block Identity) Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_IDENT0: V3D ID String (Reads As \"V3D\") READ"]
    IDSTR = 0x00FFFFFF;
    #[doc="V3D_IDENT0: V3D Technology Version (Reads Technology Version = 2) READ"]
    TVER = 0xFF000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_IDENT1: V3D Identification 1 (V3D Configuration A) Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_IDENT1: V3D Revision READ"]
    REVR = 0x0000000F;
    #[doc="V3D_IDENT1: Number Of Slices READ"]
    NSLC = 0x000000F0;
    #[doc="V3D_IDENT1: Number Of QPUs Per Slice READ"]
    QUPS = 0x00000F00;
    #[doc="V3D_IDENT1: Number Of TMUs Per Slice READ"]
    TUPS = 0x0000F000;
    #[doc="V3D_IDENT1: Number Of Semaphores READ"]
    NSEM = 0x00FF0000;
    #[doc="V3D_IDENT1: HDR Support (0 = Not Supported, 1 = Supported) READ"]
    HDRT = 0x0F000000;
    #[doc="V3D_IDENT1: VPM Memory Size (Multiples Of 1K, 0 => 16K) READ"]
    VPMSZ = 0xF0000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_IDENT2: V3D Identification 2 (V3D Configuration B) Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_IDENT2: VRI Memory Size (0 = Half Size, 1 = Full Size) READ"]
    VRISZ = 0x0000000F;
    /// V3D_IDENT2: Tile Buffer Size (0 = Quarter Size, 1 = Half Size, 2 = Full Size
    #[doc="(32x32msm)) READ"]
    TLBSZ = 0x000000F0;
    /// V3D_IDENT2: Tile Buffer Double-Buffer Mode Support (0 = Not Supported, 1 =
    #[doc="Supported) READ"]
    TLBDB = 0x00000F00;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Miscellaneous Registers
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SCRATCH: V3D Scratch Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SCRATCH: Scratch Register (Read/Write Registers For General Purposes)
    #[doc="READ/WRITE"]
    SCRATCH = 0xFFFFFFFF;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Cache Control Registers
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_L2CACTL: V3D L2 Cache Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_L2CACTL: L2 Cache Enable (Reads State Of Cache Enable Bit, Write To
    #[doc="Enable The L2 Cache) READ/WRITE"]
    L2CENA = 0x00000001;
    #[doc="V3D_L2CACTL: L2 Cache Disable (Write To Disable The L2 Cache) WRITE"]
    L2CDIS = 0x00000002;
    #[doc="V3D_L2CACTL: L2 Cache Clear (Write To Clear The L2 Cache) WRITE"]
    L2CCLR = 0x00000004;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SLCACTL: V3D Slices Cache Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SLCACTL: Instruction Cache Clear Bits (Write To Clear Instruction Cache)
    #[doc="WRITE"]
    ICCS0_TO_ICCS3 = 0x0000000F;
    #[doc="V3D_SLCACTL: Uniforms Cache Clear Bits (Write To Clear Uniforms Cache) WRITE"]
    UCCS0_TO_UCCS3 = 0x00000F00;
    #[doc="V3D_SLCACTL: TMU0 Cache Clear Bits (Write To Clear TMU0 Cache) WRITE"]
    T0CCS0_TO_T0CCS3 = 0x000F0000;
    #[doc="V3D_SLCACTL: TMU1 Cache Clear Bits (Write To Clear TMU1 Cache) WRITE"]
    T1CCS0_TO_T1CCS3 = 0x0F000000;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Pipeline Interrupt Control
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_INTCTL: V3D Interrupt Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_INTCTL: Render Mode Frame Done Interrupt Status (Set When All Tiles Of
    #[doc="The Frame Have Been Written To Memory) READ/WRITE"]
    INT_FRDONE = 0x00000001;
    /// V3D_INTCTL: Binning Mode Flush Done Interrupt Status (Set When Binning Is
    #[doc="Complete With All Tile Lists Flushed To Memory) READ/WRITE"]
    INT_FLDONE = 0x00000002;
    /// V3D_INTCTL: Binner Out Of Memory Interrupt Status (Set While The Binner
    #[doc="Needs More Memory To Complete) READ/WRITE"]
    INT_OUTOMEM = 0x00000004;
    /// V3D_INTCTL: Binner Used Overspill Memory Interrupt Status (Set When The
    #[doc="Binner Starts Using The (Valid) Overspill Memory Buffer) READ/WRITE"]
    INT_SPILLUSE = 0x00000008;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_INTENA: V3D Interrupt Enables Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_INTENA: Render Mode Frame Done Interrupt Enable (Set When The INT_FRDONE
    #[doc="Interrupt Is Set) READ/WRITE"]
    EI_FRDONE = 0x00000001;
    /// V3D_INTENA: Binning Mode Flush Done Interrupt Enable (Set When The
    #[doc="INT_FLDONE Interrupt Is Set) READ/WRITE"]
    EI_FLDONE = 0x00000002;
    /// V3D_INTENA: Binner Out Of Memory Interrupt Enable (Set When The INT_OUTOMEM
    #[doc="Interrupt Is Set) READ/WRITE"]
    EI_OUTOMEM = 0x00000004;
    /// V3D_INTENA: Binner Used Overspill Memory Interrupt Enable (Set When The
    #[doc="INT_SPILLUSE Interrupt Is Set) READ/WRITE"]
    EI_SPILLUSE = 0x00000008;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_INTDIS: V3D Interrupt Disables Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_INTDIS: Render Mode Frame Done Interrupt Disable (Set When The
    #[doc="INT_FRDONE Interrupt Is Set) READ/WRITE"]
    DI_FRDONE = 0x00000001;
    /// V3D_INTDIS: Binning Mode Flush Done Interrupt Disable (Set When The
    #[doc="INT_FLDONE Interrupt Is Set) READ/WRITE"]
    DI_FLDONE = 0x00000002;
    /// V3D_INTDIS: Binner Out Of Memory Interrupt Disable (Set When The INT_OUTOMEM
    #[doc="Interrupt Is Set) READ/WRITE"]
    DI_OUTOMEM = 0x00000004;
    /// V3D_INTDIS: Binner Used Overspill Memory Interrupt Disable (Set When The
    #[doc="INT_SPILLUSE Interrupt Is Set) READ/WRITE"]
    DI_SPILLUSE = 0x00000008;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Control List Executor Registers (Per Thread)
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnCS: V3D Control List Executor Thread n Control & Status Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_CTnCS: Control Thread Mode (Binning Mode Thread Only) READ"]
    CTMODE = 0x00000001;
    /// V3D_CTnCS: Control Thread Error (Set When Stopped With An Error, Cleared On
    #[doc="Restarting) READ"]
    CTERR = 0x00000008;
    #[doc="V3D_CTnCS: Control Thread Sub-Mode READ/WRITE"]
    CTSUBS = 0x00000010;
    #[doc="V3D_CTnCS: Control Thread Run READ/WRITE"]
    CTRUN = 0x00000020;
    #[doc="V3D_CTnCS: Return Stack Depth (Number Of Levels Of List Nesting) READ"]
    CTRTSD = 0x00000300;
    /// V3D_CTnCS: Counting Semaphore (Current State Of The Counting Semaphore For
    #[doc="This Thread) READ"]
    CTSEMA = 0x00007000;
    /// V3D_CTnCS: Reset Bit (Writing 1 Stops The Control Thread & Resets All Bits
    #[doc="In This Register) WRITE"]
    CTRSTA = 0x00008000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnEA: V3D Control List Executor Thread n End Address Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_CTnEA: Control List End Address (Set To The Byte Address After The Last
    #[doc="Record In The Control List) READ/WRITE"]
    CTLEA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnCA: V3D Control List Executor Thread n Current Address Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_CTnCA: Control List Current Address (Points To The Address Of The
    #[doc="Current Record In The Control List) READ/WRITE"]
    CTLCA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnRA0: V3D Control List Executor Thread n Return Address Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_CTnRA0: Control List Return Address 0 (Address On Return Address Stack)
    #[doc="READ"]
    CTLRA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnLC: V3D Control List Executor Thread n List Counter Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_CTnLC: Sub-list Counter (Count Of Return Commands Encountered)
    #[doc="READ/WRITE"]
    CTLSLCS = 0x0000FFFF;
    /// V3D_CTnLC: Major List Counter (Count Of Flush Commands Encountered)
    #[doc="READ/WRITE"]
    CTLLCM = 0xFFFF0000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_CTnPC: V3D Control List Executor Thread n Primitive List Counter Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_CTnPC: Primitive List Counter (Count Of Primitives Remaining Whilst
    #[doc="Processing A Primitive List) READ"]
    CTLPC = 0xFFFFFFFF;

    //;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Pipeline Registers
    //;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_PCS: V3D Pipeline Control & Status Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_PCS: Binning Mode Active (Set While Binning Pipeline Is In Use) READ"]
    BMACTIVE = 0x00000001;
    /// V3D_PCS: Binning Mode Busy (Set While Any Binning Operations Are Actually In
    #[doc="Progress) READ"]
    BMBUSY = 0x00000002;
    #[doc="V3D_PCS: Rendering Mode Active (Set While Rendering Pipeline Is In Use) READ"]
    RMACTIVE = 0x00000004;
    /// V3D_PCS: Rendering Mode Busy (Set While Any Rendering Operations Are
    #[doc="Actually In Progress) READ"]
    RMBUSY = 0x00000008;
    /// V3D_PCS: Binning Mode Out Of Memory (Set When PTB Runs Out Of Binning Memory
    #[doc="While Binning) READ"]
    BMOOM = 0x00000100;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BFC: V3D Binning Mode Flush Count Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_BFC: Flush Count (Count Increments In Binning Mode Once PTB Has Flushed
    /// All Tile Lists To Mem & PTB Has Finished With Tile State Data Array)
    #[doc="READ/WRITE"]
    BMFCT = 0x000000FF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_RFC: V3D Rendering Mode Frame Count Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_RFC: Frame Count (Count Increments In Rendering Mode When Last Tile
    /// Store Operation Of Frame Completes, The Tile Has Fully Written Out To Mem)
    #[doc="READ/WRITE"]
    RMFCT = 0x000000FF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BPCA: V3D Current Address Of Binning Memory Pool Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_BPCA: Current Pool Address (The Address Of The Current Allocation
    #[doc="Pointer In The Binning Memory Pool) READ"]
    BMPCA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BPCS: V3D Remaining Size Of Binning Memory Pool Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_BPCS: Size Of Pool Remaining (The Number Of Bytes Remaining In The
    #[doc="Binning Memory Pool) READ"]
    BMPRS = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BPOA: V3D Address Of Overspill Binning Memory Block Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_BPOA: Address Of Overspill Memory Block For Binning (Address Of
    /// Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out)
    #[doc="READ/WRITE"]
    BMPOA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BPOS: V3D Size Of Overspill Binning Memory Block Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_BPOS: Size Of Overspill Memory Block For Binning (Number Of Bytes Of
    /// Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out)
    #[doc="READ/WRITE"]
    BMPOS = 0xFFFFFFFF;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D QPU Scheduler Registers
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SQRSV0: V3D Reserve QPUs 0-7 Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_SQRSV0: Reservation Settings For QPU 0 READ/WRITE"]
    QPURSV0 = 0x0000000F;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 1 READ/WRITE"]
    QPURSV1 = 0x000000F0;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 2 READ/WRITE"]
    QPURSV2 = 0x00000F00;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 3 READ/WRITE"]
    QPURSV3 = 0x0000F000;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 4 READ/WRITE"]
    QPURSV4 = 0x000F0000;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 5 READ/WRITE"]
    QPURSV5 = 0x00F00000;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 6 READ/WRITE"]
    QPURSV6 = 0x0F000000;
    #[doc="V3D_SQRSV0: Reservation Settings For QPU 7 READ/WRITE"]
    QPURSV7 = 0xF0000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SQRSV1: V3D Reserve QPUs 8-15 Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_SQRSV1: Reservation Settings For QPU 8 READ/WRITE"]
    QPURSV8 = 0x0000000F;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 9 READ/WRITE"]
    QPURSV9 = 0x000000F0;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 10 READ/WRITE"]
    QPURSV10 = 0x00000F00;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 11 READ/WRITE"]
    QPURSV11 = 0x0000F000;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 12 READ/WRITE"]
    QPURSV12 = 0x000F0000;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 13 READ/WRITE"]
    QPURSV13 = 0x00F00000;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 14 READ/WRITE"]
    QPURSV14 = 0x0F000000;
    #[doc="V3D_SQRSV1: Reservation Settings For QPU 15 READ/WRITE"]
    QPURSV15 = 0xF0000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SQCNTL: V3D QPU Scheduler Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_SQCNTL: Vertex Shader Scheduling Bypass Limit READ/WRITE"]
    VSRBL = 0x00000003;
    #[doc="V3D_SQCNTL: Coordinate Shader Scheduling Bypass Limit READ/WRITE"]
    CSRBL = 0x0000000C;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SRQPC: V3D QPU User Program Request Program Address Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SRQPC: Program Address (Writing This Register Queues A Request To Run A
    #[doc="Program Starting At The Given Address) WRITE"]
    QPURQPC = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SRQUA: V3D QPU User Program Request Uniforms Address Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SRQUA: Uniforms Address (Contains The Address Of The Uniforms Stream For
    #[doc="The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE"]
    QPURQUA = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SRQUL: V3D QPU User Program Request Uniforms Length Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SRQUL: Uniforms Length (Contains The Max Length Of The Uniforms Stream
    #[doc="For The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE"]
    QPURQUL = 0x00000FFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_SRQCS: V3D QPU User Program Request Control & Status Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_SRQCS: Queue Length (Contains The Number Of Program Requests Currently
    #[doc="Queued) READ/WRITE"]
    QPURQL = 0x0000003F;
    /// V3D_SRQCS: Queue Error (Set When A Request Has Been Made When The Queue Is
    #[doc="Full) READ/WRITE"]
    QPURQERR = 0x00000080;
    /// V3D_SRQCS: Count Of User Program Requests Made (Contains The Total Number Of
    #[doc="User Program Requests Made, Modulo 256) READ/WRITE"]
    QPURQCM = 0x0000FF00;
    /// V3D_SRQCS: Count Of User Programs Completed (Contains The Total Number Of
    #[doc="User Programs That Have Run & Completed, Modulo 256) READ/WRITE"]
    QPURQCC = 0x00FF0000;

    //;;;;;;;;;;;;;;;;;;;
    // V3D VPM Registers
    //;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_VPACNTL: V3D VPM Allocator Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_VPACNTL: Rendering VPM Allocation Limit (Limits The Amount Of VPM Memory
    #[doc="Allocated To Rendering Mode) READ/WRITE"]
    VPARALIM = 0x00000007;
    /// V3D_VPACNTL: Binning VPM Allocation Limit (Limits The Amount Of VPM Memory
    #[doc="Allocated To Binning Mode) READ/WRITE"]
    VPABALIM = 0x00000038;
    /// V3D_VPACNTL: Rendering VPM Allocation Timeout (Sets A Timeout For Raising
    #[doc="The Priority Of Rendering Mode Allocation Requests) READ/WRITE"]
    VPARATO = 0x000001C0;
    /// V3D_VPACNTL: Binning VPM Allocation Timeout (Sets A Timeout For Raising The
    #[doc="Priority Of Binning Mode Allocation Requests) READ/WRITE"]
    VPABATO = 0x00000E00;
    /// V3D_VPACNTL: Enable VPM Allocation Limits (Enables VPM Memory Allocation
    #[doc="Limiting Using VPARALIM & VPABALIM) READ/WRITE"]
    VPALIMEN = 0x00001000;
    /// V3D_VPACNTL: Enable VPM Allocation Timeout (Enables VPM Memory Allocation
    #[doc="Timeout Using VPARATO & VPABATO) READ/WRITE"]
    VPATOEN = 0x00002000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_VPMBASE: V3D VPM Base (User) Memory Reservation Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_VPMBASE: VPM Memory Reserved For User Programs (Contains Amount Of VPM
    #[doc="Mem Reserved For All User Programs, In Multiples Of 256 Bytes) READ/WRITE"]
    VPMURSV = 0x0000001F;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D QPU Interrupt Control
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_DBQITE: V3D QPU Interrupt Enables Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_DBQITE: QPU Interrupt Enable bits (Set Bit To Allow QPU To Generate An
    #[doc="Interrupt) READ/WRITE"]
    IE_QPU0_TO_IE_QPU15 = 0x0000FFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_DBQITC: V3D QPU Interrupt Control Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_DBQITC: QPU Interrupt Control Bits (Reads When Interrupt Is Latched,
    #[doc="Write To Clear Interrupt) READ/WRITE"]
    IC_QPU0_TO_IC_QPU15 = 0x0000FFFF;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Performance Counters
    //;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D Sources For Performance Counters
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// FEP Valid Primitives That Result In No Rendered Pixels, For All Rendered
    #[doc="Tiles"]
    COUNT_ID_0 = 0;
    /// FEP Valid Primitives For All Rendered Tiles (Primitives May Be Counted In
    #[doc="More Than One Tile)"]
    COUNT_ID_1 = 1;
    #[doc="FEP Early-Z/Near/Far Clipped Quads"]
    COUNT_ID_2 = 2;
    #[doc="FEP Valid Quads"]
    COUNT_ID_3 = 3;
    #[doc="TLB Quads With No Pixels Passing The Stencil Test"]
    COUNT_ID_4 = 4;
    #[doc="TLB Quads With No Pixels Passing The Z & Stencil Tests"]
    COUNT_ID_5 = 5;
    #[doc="TLB Quads With Any Pixels Passing The Z & Stencil Tests"]
    COUNT_ID_6 = 6;
    #[doc="TLB Quads With All Pixels Having Zero Coverage"]
    COUNT_ID_7 = 7;
    #[doc="TLB Quads With Any Pixels Having Non-Zero Coverage"]
    COUNT_ID_8 = 8;
    #[doc="TLB Quads With Valid Pixels Written To Color Buffer"]
    COUNT_ID_9 = 9;
    #[doc="PTB Primitives Discarded By Being Outside The Viewport"]
    COUNT_ID_10 = 10;
    #[doc="PTB Primitives That Need Clipping"]
    COUNT_ID_11 = 11;
    #[doc="PSE Primitives That Are Discarded Because They Are Reversed"]
    COUNT_ID_12 = 12;
    #[doc="QPU Total Idle Clock Cycles For All QPUs"]
    COUNT_ID_13 = 13;
    #[doc="QPU Total Clock Cycles For All QPUs Doing Vertex/Coordinate Shading"]
    COUNT_ID_14 = 14;
    #[doc="QPU Total Clock Cycles For All QPUs Doing Fragment Shading"]
    COUNT_ID_15 = 15;
    #[doc="QPU Total Clock Cycles For All QPUs Executing Valid Instructions"]
    COUNT_ID_16 = 16;
    #[doc="QPU Total Clock Cycles For All QPUs Stalled Waiting For TMUs"]
    COUNT_ID_17 = 17;
    #[doc="QPU Total Clock Cycles For All QPUs Stalled Waiting For Scoreboard"]
    COUNT_ID_18 = 18;
    #[doc="QPU Total Clock Cycles For All QPUs Stalled Waiting For Varyings"]
    COUNT_ID_19 = 19;
    #[doc="QPU Total Instruction Cache Hits For All Slices"]
    COUNT_ID_20 = 20;
    #[doc="QPU Total Instruction Cache Misses For All Slices"]
    COUNT_ID_21 = 21;
    #[doc="QPU Total Uniforms Cache Hits For All Slices"]
    COUNT_ID_22 = 22;
    #[doc="QPU Total Uniforms Cache Misses For All Slices"]
    COUNT_ID_23 = 23;
    #[doc="TMU Total Texture Quads Processed"]
    COUNT_ID_24 = 24;
    #[doc="TMU Total Texture Cache Misses (Number Of Fetches From Memory/L2 Cache)"]
    COUNT_ID_25 = 25;
    #[doc="VPM Total Clock Cycles VDW Is Stalled Waiting For VPM Access"]
    COUNT_ID_26 = 26;
    #[doc="VPM Total Clock Cycles VCD Is Stalled Waiting For VPM Access"]
    COUNT_ID_27 = 27;
    #[doc="L2C Total Level 2 Cache Hits"]
    COUNT_ID_28 = 28;
    #[doc="L2C Total Level 2 Cache Misses"]
    COUNT_ID_29 = 29;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_PCTRC: V3D Performance Counter Clear Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_PCTRC: Performance Counter Clear Bits (Write To Clear The Performance
    #[doc="Counter) WRITE"]
    CTCLR0_CTCLR15 = 0x0000FFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_PCTRE: V3D Performance Counter Enables Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    /// V3D_PCTRE: Performance Counter Enable Bits (0 = Counter Disabled, 1 =
    #[doc="Performance Counter Enabled To Count) READ/WRITE"]
    CTEN0_CTEN15 = 0x0000FFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_PCTRn: V3D Performance Counter Count n Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_PCTRn: Performance Count (Count Value) READ/WRITE"]
    PCTR = 0xFFFFFFFF;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_PCTRSn: V3D Performance Counter Mapping n Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_PCTRSn: Performance Counter Device ID READ/WRITE"]
    PCTRS = 0x0000001F;

    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    // V3D Error & Diagnostic Registers
    //;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_BXCF: V3D Binner Debug Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_BXCF: Disable Forwarding In State Cache READ/WRITE"]
    FWDDISA = 0x00000001;
    #[doc="V3D_BXCF: Disable Clipping READ/WRITE"]
    CLIPDISA = 0x00000002;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_DBGE: V3D PSE Error Signals Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_DBGE: Error A Reading VPM READ"]
    VR1_A = 0x00000002;
    #[doc="V3D_DBGE: Error B Reading VPM READ"]
    VR1_B = 0x00000004;
    #[doc="V3D_DBGE: Error Mulip 0 READ"]
    MULIP0 = 0x00010000;
    #[doc="V3D_DBGE: Error Mulip 1 READ"]
    MULIP1 = 0x00020000;
    #[doc="V3D_DBGE: Error Mulip 2 READ"]
    MULIP2 = 0x00040000;
    #[doc="V3D_DBGE: Error IPD2 Valid READ"]
    IPD2_VALID = 0x00080000;
    #[doc="V3D_DBGE: Error IPD2 FPD Used READ"]
    IPD2_FPDUSED = 0x00100000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_FDBGO: V3D FEP Overrun Error Signals Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_FDBGO: Not An Error READ"]
    WCOEFF_FIFO_FULL = 0x00000002;
    #[doc="V3D_FDBGO: Not An Error READ"]
    XYRELZ_FIFO_FULL = 0x00000004;
    #[doc="V3D_FDBGO: Error READ"]
    QBFR_FIFO_ORUN = 0x00000008;
    #[doc="V3D_FDBGO: Error READ"]
    QBSZ_FIFO_ORUN = 0x00000010;
    #[doc="V3D_FDBGO: Error READ"]
    XYFO_FIFO_ORUN = 0x00000020;
    #[doc="V3D_FDBGO: Error READ"]
    FIXZ_ORUN = 0x00000040;
    #[doc="V3D_FDBGO: Error READ"]
    XYRELO_FIFO_ORUN = 0x00000080;
    #[doc="V3D_FDBGO: Error READ"]
    XYRELW_FIFO_ORUN = 0x00000400;
    #[doc="V3D_FDBGO: Not An Error"]
    ZCOEFF_FIFO_FULL = 0x00000800;
    #[doc="V3D_FDBGO: Error READ"]
    REFXY_FIFO_ORUN = 0x00001000;
    #[doc="V3D_FDBGO: Error READ"]
    DEPTHO_FIFO_ORUN = 0x00002000;
    #[doc="V3D_FDBGO: Error READ"]
    DEPTHO_ORUN = 0x00004000;
    #[doc="V3D_FDBGO: Error READ"]
    EZVAL_FIFO_ORUN = 0x00008000;
    #[doc="V3D_FDBGO: Error READ"]
    EZREQ_FIFO_ORUN = 0x00020000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_FDBGB: V3D FEP Interface Ready & Stall Signals, FEP Busy Signals Register
    // Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_FDBGB: Stall READ"]
    EDGES_STALL = 0x00000001;
    #[doc="V3D_FDBGB: Ready READ"]
    EDGES_READY = 0x00000002;
    #[doc="V3D_FDBGB: READ"]
    EDGES_ISCTRL = 0x00000004;
    #[doc="V3D_FDBGB: READ"]
    EDGES_CTRLID = 0x00000038;
    #[doc="V3D_FDBGB: Stall READ"]
    ZRWPE_STALL = 0x00000040;
    #[doc="V3D_FDBGB: Ready READ"]
    ZRWPE_READY = 0x00000080;
    #[doc="V3D_FDBGB: Ready READ"]
    EZ_DATA_READY = 0x00800000;
    #[doc="V3D_FDBGB: Ready READ"]
    EZ_XY_READY = 0x02000000;
    #[doc="V3D_FDBGB: Busy READ"]
    RAST_BUSY = 0x04000000;
    #[doc="V3D_FDBGB: Ready READ"]
    QXYF_FIFO_OP_READY = 0x08000000;
    #[doc="V3D_FDBGB: Ready READ"]
    XYFO_FIFO_OP_READY = 0x10000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_FDBGR: V3D FEP Internal Ready Signals Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_FDBGR: Ready READ"]
    QXYF_FIFO_READY = 0x00000001;
    #[doc="V3D_FDBGR: Ready READ"]
    EZREQ_FIFO_READY = 0x00000002;
    #[doc="V3D_FDBGR: Ready READ"]
    EZVAL_FIFO_READY = 0x00000004;
    #[doc="V3D_FDBGR: Ready READ"]
    DEPTHO_FIFO_READY = 0x00000008;
    #[doc="V3D_FDBGR: Ready READ"]
    REFXY_FIFO_READY = 0x00000010;
    #[doc="V3D_FDBGR: Ready READ"]
    ZCOEFF_FIFO_READY = 0x00000020;
    #[doc="V3D_FDBGR: Ready READ"]
    XYRELW_FIFO_READY = 0x00000040;
    #[doc="V3D_FDBGR: Ready READ"]
    WCOEFF_FIFO_READY = 0x00000080;
    #[doc="V3D_FDBGR: Ready READ"]
    XYRELO_FIFO_READY = 0x00000800;
    #[doc="V3D_FDBGR: Ready READ"]
    ZO_FIFO_READY = 0x00002000;
    #[doc="V3D_FDBGR: Ready READ"]
    XYFO_FIFO_READY = 0x00004000;
    #[doc="V3D_FDBGR: Ready READ"]
    RAST_READY = 0x00010000;
    #[doc="V3D_FDBGR: Last READ"]
    RAST_LAST = 0x00020000;
    #[doc="V3D_FDBGR: Ready READ"]
    DEPTHO_READY = 0x00040000;
    #[doc="V3D_FDBGR: Ready READ"]
    EZLIM_READY = 0x00080000;
    #[doc="V3D_FDBGR: Ready READ"]
    XYNRM_READY = 0x00100000;
    #[doc="V3D_FDBGR: Last READ"]
    XYNRM_LAST = 0x00200000;
    #[doc="V3D_FDBGR: Ready READ"]
    XYRELZ_FIFO_READY = 0x00400000;
    #[doc="V3D_FDBGR: Last READ"]
    XYRELZ_FIFO_LAST = 0x00800000;
    #[doc="V3D_FDBGR: Ready READ"]
    INTERPZ_READY = 0x01000000;
    #[doc="V3D_FDBGR: Ready READ"]
    INTERPRW_READY = 0x08000000;
    #[doc="V3D_FDBGR: Ready READ"]
    RECIPW_READY = 0x10000000;
    #[doc="V3D_FDBGR: Ready READ"]
    FIXZ_READY = 0x40000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_FDBGS: V3D FEP Internal Stall Input Signals Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_FDBGS: Stall READ"]
    EZTEST_IP_QSTALL = 0x00000001;
    #[doc="V3D_FDBGS: Stall READ"]
    EZTEST_IP_PRSTALL = 0x00000002;
    #[doc="V3D_FDBGS: Stall READ"]
    EZTEST_IP_VLFSTALL = 0x00000004;
    #[doc="V3D_FDBGS: Stall READ"]
    EZTEST_STALL = 0x00000008;
    #[doc="V3D_FDBGS: Valid READ"]
    EZTEST_VLF_OKNOVALID = 0x00000010;
    #[doc="V3D_FDBGS: Ready READ"]
    EZTEST_QREADY = 0x00000020;
    #[doc="V3D_FDBGS: READ"]
    EZTEST_ANYQF = 0x00000040;
    #[doc="V3D_FDBGS: Valid READ"]
    EZTEST_ANYQVALID = 0x00000080;
    #[doc="V3D_FDBGS: Valid READ"]
    QXYF_FIFO_OP1_VALID = 0x00000100;
    #[doc="V3D_FDBGR: Last READ"]
    QXYF_FIFO_OP1_LAST = 0x00000200;
    #[doc="V3D_FDBGR: Dummy READ"]
    QXYF_FIFO_OP1_DUMMY = 0x00000400;
    #[doc="V3D_FDBGR: Last READ"]
    QXYF_FIFO_OP_LAST = 0x00000800;
    #[doc="V3D_FDBGS: Valid READ"]
    QXYF_FIFO_OP_VALID = 0x00001000;
    #[doc="V3D_FDBGS: Valid READ"]
    EZREQ_FIFO_OP_VALID = 0x00002000;
    #[doc="V3D_FDBGS: Stall READ"]
    XYNRM_IP_STALL = 0x00004000;
    #[doc="V3D_FDBGS: Stall READ"]
    EZLIM_IP_STALL = 0x00008000;
    #[doc="V3D_FDBGS: Stall READ"]
    DEPTHO_FIFO_IP_STALL = 0x00010000;
    #[doc="V3D_FDBGS: Stall READ"]
    INTERPZ_IP_STALL = 0x00020000;
    #[doc="V3D_FDBGS: Stall READ"]
    XYRELZ_FIFO_IP_STALL = 0x00040000;
    #[doc="V3D_FDBGS: Stall READ"]
    INTERPW_IP_STALL = 0x00400000;
    #[doc="V3D_FDBGS: Stall READ"]
    RECIPW_IP_STALL = 0x02000000;
    #[doc="V3D_FDBGS: Stall READ"]
    ZO_FIFO_IP_STALL = 0x10000000;

    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
    // V3D_ERRSTAT: V3D Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C)
    // Register Description
    // * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

    #[doc="V3D_ERRSTAT: VPM Allocator Error - Allocating Base While Busy READ"]
    VPAEABB = 0x00000001;
    #[doc="V3D_ERRSTAT: VPM Allocator Error - Request Too Big READ"]
    VPAERGS = 0x00000002;
    #[doc="V3D_ERRSTAT: VPM Allocator Error - Binner Request Greater Than Limit READ"]
    VPAEBRGL = 0x00000004;
    #[doc="V3D_ERRSTAT: VPM Allocator Error - Renderer Request Greater Than Limit READ"]
    VPAERRGL = 0x00000008;
    #[doc="V3D_ERRSTAT: VPM Error - Write Range READ"]
    VPMEWR = 0x00000010;
    #[doc="V3D_ERRSTAT: VPM Error - Read Range READ"]
    VPMERR = 0x00000020;
    #[doc="V3D_ERRSTAT: VPM Error - Read Non-Allocated READ"]
    VPMERNA = 0x00000040;
    #[doc="V3D_ERRSTAT: VPM Error - Write Non-Allocated READ"]
    VPMEWNA = 0x00000080;
    #[doc="V3D_ERRSTAT: VPM Error - Free Non-Allocated READ"]
    VPMEFNA = 0x00000100;
    #[doc="V3D_ERRSTAT: VPM Error - Allocated Size Error READ"]
    VPMEAS = 0x00000200;
    #[doc="V3D_ERRSTAT: VDW Error - Address Overflows READ"]
    VDWE = 0x00000400;
    #[doc="V3D_ERRSTAT: VCD Error - FIFO Pointers Out Of Sync READ"]
    VCDE = 0x00000800;
    #[doc="V3D_ERRSTAT: VCD Idle READ"]
    VCDI = 0x00001000;
    #[doc="V3D_ERRSTAT: VCM Error (Renderer) READ"]
    VCMRE = 0x00002000;
    #[doc="V3D_ERRSTAT: VCM Error (Binner) READ"]
    VCMBE = 0x00004000;
    #[doc="V3D_ERRSTAT: L2C AXI Receive Fifo Overrun Error READ"]
    L2CARE = 0x00008000;
}
