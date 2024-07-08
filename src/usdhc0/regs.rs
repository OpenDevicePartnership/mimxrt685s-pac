#[doc = "ADMA Error Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaErrStatus(pub u32);
impl AdmaErrStatus {
    #[doc = "ADMA Error State (when ADMA Error is occurred)"]
    #[inline(always)]
    pub const fn admaes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADMA Error State (when ADMA Error is occurred)"]
    #[inline(always)]
    pub fn set_admaes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADMA Length Mismatch Error"]
    #[inline(always)]
    pub const fn admalme(&self) -> super::vals::Admalme {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Admalme::from_bits(val as u8)
    }
    #[doc = "ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn set_admalme(&mut self, val: super::vals::Admalme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ADMA Descriptor Error"]
    #[inline(always)]
    pub const fn admadce(&self) -> super::vals::Admadce {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Admadce::from_bits(val as u8)
    }
    #[doc = "ADMA Descriptor Error"]
    #[inline(always)]
    pub fn set_admadce(&mut self, val: super::vals::Admadce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for AdmaErrStatus {
    #[inline(always)]
    fn default() -> AdmaErrStatus {
        AdmaErrStatus(0)
    }
}
#[doc = "ADMA System Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaSysAddr(pub u32);
impl AdmaSysAddr {
    #[doc = "ADMA System Address"]
    #[inline(always)]
    pub const fn ads_addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "ADMA System Address"]
    #[inline(always)]
    pub fn set_ads_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for AdmaSysAddr {
    #[inline(always)]
    fn default() -> AdmaSysAddr {
        AdmaSysAddr(0)
    }
}
#[doc = "Auto CMD12 Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autocmd12ErrStatus(pub u32);
impl Autocmd12ErrStatus {
    #[doc = "Auto CMD12 Not Executed"]
    #[inline(always)]
    pub const fn ac12ne(&self) -> super::vals::Ac12ne {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ac12ne::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn set_ac12ne(&mut self, val: super::vals::Ac12ne) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Auto CMD12 / 23 Timeout Error"]
    #[inline(always)]
    pub const fn ac12toe(&self) -> super::vals::Ac12toe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ac12toe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 Timeout Error"]
    #[inline(always)]
    pub fn set_ac12toe(&mut self, val: super::vals::Ac12toe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 / 23 End Bit Error"]
    #[inline(always)]
    pub const fn ac12ebe(&self) -> super::vals::Ac12ebe {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ac12ebe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 End Bit Error"]
    #[inline(always)]
    pub fn set_ac12ebe(&mut self, val: super::vals::Ac12ebe) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto CMD12 / 23 CRC Error"]
    #[inline(always)]
    pub const fn ac12ce(&self) -> super::vals::Ac12ce {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ac12ce::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 CRC Error"]
    #[inline(always)]
    pub fn set_ac12ce(&mut self, val: super::vals::Ac12ce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Auto CMD12 / 23 Index Error"]
    #[inline(always)]
    pub const fn ac12ie(&self) -> super::vals::Ac12ie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ac12ie::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 Index Error"]
    #[inline(always)]
    pub fn set_ac12ie(&mut self, val: super::vals::Ac12ie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub const fn cnibac12e(&self) -> super::vals::Cnibac12e {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cnibac12e::from_bits(val as u8)
    }
    #[doc = "Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn set_cnibac12e(&mut self, val: super::vals::Cnibac12e) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub const fn execute_tuning(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn set_execute_tuning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Sample Clock Select"]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> super::vals::Autocmd12ErrStatusSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Autocmd12ErrStatusSmpClkSel::from_bits(val as u8)
    }
    #[doc = "Sample Clock Select"]
    #[inline(always)]
    pub fn set_smp_clk_sel(&mut self, val: super::vals::Autocmd12ErrStatusSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Autocmd12ErrStatus {
    #[inline(always)]
    fn default() -> Autocmd12ErrStatus {
        Autocmd12ErrStatus(0)
    }
}
#[doc = "Block Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkAtt(pub u32);
impl BlkAtt {
    #[doc = "Block Size"]
    #[inline(always)]
    pub const fn blksize(&self) -> super::vals::Blksize {
        let val = (self.0 >> 0usize) & 0x1fff;
        super::vals::Blksize::from_bits(val as u16)
    }
    #[doc = "Block Size"]
    #[inline(always)]
    pub fn set_blksize(&mut self, val: super::vals::Blksize) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val.to_bits() as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Block Count"]
    #[inline(always)]
    pub const fn blkcnt(&self) -> super::vals::Blkcnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Blkcnt::from_bits(val as u16)
    }
    #[doc = "Block Count"]
    #[inline(always)]
    pub fn set_blkcnt(&mut self, val: super::vals::Blkcnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for BlkAtt {
    #[inline(always)]
    fn default() -> BlkAtt {
        BlkAtt(0)
    }
}
#[doc = "CLK Tuning Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTuneCtrlStatus(pub u32);
impl ClkTuneCtrlStatus {
    #[doc = "DLY_CELL_SET_POST"]
    #[inline(always)]
    pub const fn dly_cell_set_post(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DLY_CELL_SET_POST"]
    #[inline(always)]
    pub fn set_dly_cell_set_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub const fn dly_cell_set_out(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub fn set_dly_cell_set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub const fn dly_cell_set_pre(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub fn set_dly_cell_set_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "NXT_ERR"]
    #[inline(always)]
    pub const fn nxt_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NXT_ERR"]
    #[inline(always)]
    pub fn set_nxt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "TAP_SEL_POST"]
    #[inline(always)]
    pub const fn tap_sel_post(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "TAP_SEL_POST"]
    #[inline(always)]
    pub fn set_tap_sel_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "TAP_SEL_OUT"]
    #[inline(always)]
    pub const fn tap_sel_out(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "TAP_SEL_OUT"]
    #[inline(always)]
    pub fn set_tap_sel_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "TAP_SEL_PRE"]
    #[inline(always)]
    pub const fn tap_sel_pre(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "TAP_SEL_PRE"]
    #[inline(always)]
    pub fn set_tap_sel_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "PRE_ERR"]
    #[inline(always)]
    pub const fn pre_err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PRE_ERR"]
    #[inline(always)]
    pub fn set_pre_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkTuneCtrlStatus {
    #[inline(always)]
    fn default() -> ClkTuneCtrlStatus {
        ClkTuneCtrlStatus(0)
    }
}
#[doc = "Command Argument"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdArg(pub u32);
impl CmdArg {
    #[doc = "Command Argument"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Argument"]
    #[inline(always)]
    pub fn set_cmdarg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdArg {
    #[inline(always)]
    fn default() -> CmdArg {
        CmdArg(0)
    }
}
#[doc = "Command Response0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp0(pub u32);
impl CmdRsp0 {
    #[doc = "Command Response 0"]
    #[inline(always)]
    pub const fn cmdrsp0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Response 0"]
    #[inline(always)]
    pub fn set_cmdrsp0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp0 {
    #[inline(always)]
    fn default() -> CmdRsp0 {
        CmdRsp0(0)
    }
}
#[doc = "Command Response1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp1(pub u32);
impl CmdRsp1 {
    #[doc = "Command Response 1"]
    #[inline(always)]
    pub const fn cmdrsp1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Response 1"]
    #[inline(always)]
    pub fn set_cmdrsp1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp1 {
    #[inline(always)]
    fn default() -> CmdRsp1 {
        CmdRsp1(0)
    }
}
#[doc = "Command Response2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp2(pub u32);
impl CmdRsp2 {
    #[doc = "Command Response 2"]
    #[inline(always)]
    pub const fn cmdrsp2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Response 2"]
    #[inline(always)]
    pub fn set_cmdrsp2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp2 {
    #[inline(always)]
    fn default() -> CmdRsp2 {
        CmdRsp2(0)
    }
}
#[doc = "Command Response3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp3(pub u32);
impl CmdRsp3 {
    #[doc = "Command Response 3"]
    #[inline(always)]
    pub const fn cmdrsp3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Response 3"]
    #[inline(always)]
    pub fn set_cmdrsp3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp3 {
    #[inline(always)]
    fn default() -> CmdRsp3 {
        CmdRsp3(0)
    }
}
#[doc = "Command Transfer Type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdXfrTyp(pub u32);
impl CmdXfrTyp {
    #[doc = "Response Type Select"]
    #[inline(always)]
    pub const fn rsptyp(&self) -> super::vals::Rsptyp {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Rsptyp::from_bits(val as u8)
    }
    #[doc = "Response Type Select"]
    #[inline(always)]
    pub fn set_rsptyp(&mut self, val: super::vals::Rsptyp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command CRC Check Enable"]
    #[inline(always)]
    pub const fn cccen(&self) -> super::vals::Cccen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cccen::from_bits(val as u8)
    }
    #[doc = "Command CRC Check Enable"]
    #[inline(always)]
    pub fn set_cccen(&mut self, val: super::vals::Cccen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Command Index Check Enable"]
    #[inline(always)]
    pub const fn cicen(&self) -> super::vals::Cicen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Cicen::from_bits(val as u8)
    }
    #[doc = "Command Index Check Enable"]
    #[inline(always)]
    pub fn set_cicen(&mut self, val: super::vals::Cicen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data Present Select"]
    #[inline(always)]
    pub const fn dpsel(&self) -> super::vals::Dpsel {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dpsel::from_bits(val as u8)
    }
    #[doc = "Data Present Select"]
    #[inline(always)]
    pub fn set_dpsel(&mut self, val: super::vals::Dpsel) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Command Type"]
    #[inline(always)]
    pub const fn cmdtyp(&self) -> super::vals::Cmdtyp {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Cmdtyp::from_bits(val as u8)
    }
    #[doc = "Command Type"]
    #[inline(always)]
    pub fn set_cmdtyp(&mut self, val: super::vals::Cmdtyp) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Command Index"]
    #[inline(always)]
    pub const fn cmdinx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Command Index"]
    #[inline(always)]
    pub fn set_cmdinx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for CmdXfrTyp {
    #[inline(always)]
    fn default() -> CmdXfrTyp {
        CmdXfrTyp(0)
    }
}
#[doc = "Data Buffer Access Port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataBuffAccPort(pub u32);
impl DataBuffAccPort {
    #[doc = "Data Content"]
    #[inline(always)]
    pub const fn datcont(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Content"]
    #[inline(always)]
    pub fn set_datcont(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DataBuffAccPort {
    #[inline(always)]
    fn default() -> DataBuffAccPort {
        DataBuffAccPort(0)
    }
}
#[doc = "DLL (Delay Line) Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllCtrl(pub u32);
impl DllCtrl {
    #[doc = "DLL_CTRL_ENABLE"]
    #[inline(always)]
    pub const fn dll_ctrl_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_CTRL_ENABLE"]
    #[inline(always)]
    pub fn set_dll_ctrl_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL_CTRL_RESET"]
    #[inline(always)]
    pub const fn dll_ctrl_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_CTRL_RESET"]
    #[inline(always)]
    pub fn set_dll_ctrl_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_force_upd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_force_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target0(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_dly_target0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    pub const fn dll_ctrl_gate_update(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    pub fn set_dll_ctrl_gate_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override_val(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_override_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_dly_target1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    pub const fn dll_ctrl_slv_update_int(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    pub fn set_dll_ctrl_slv_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    pub const fn dll_ctrl_ref_update_int(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    pub fn set_dll_ctrl_ref_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for DllCtrl {
    #[inline(always)]
    fn default() -> DllCtrl {
        DllCtrl(0)
    }
}
#[doc = "DLL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllStatus(pub u32);
impl DllStatus {
    #[doc = "DLL_STS_SLV_LOCK"]
    #[inline(always)]
    pub const fn dll_sts_slv_lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_STS_SLV_LOCK"]
    #[inline(always)]
    pub fn set_dll_sts_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL_STS_REF_LOCK"]
    #[inline(always)]
    pub const fn dll_sts_ref_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DLL_STS_REF_LOCK"]
    #[inline(always)]
    pub fn set_dll_sts_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL_STS_SLV_SEL"]
    #[inline(always)]
    pub const fn dll_sts_slv_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "DLL_STS_SLV_SEL"]
    #[inline(always)]
    pub fn set_dll_sts_slv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
    #[doc = "DLL_STS_REF_SEL"]
    #[inline(always)]
    pub const fn dll_sts_ref_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "DLL_STS_REF_SEL"]
    #[inline(always)]
    pub fn set_dll_sts_ref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
}
impl Default for DllStatus {
    #[inline(always)]
    fn default() -> DllStatus {
        DllStatus(0)
    }
}
#[doc = "DMA System Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsAddr(pub u32);
impl DsAddr {
    #[doc = "DS_ADDR"]
    #[inline(always)]
    pub const fn ds_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DS_ADDR"]
    #[inline(always)]
    pub fn set_ds_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DsAddr {
    #[inline(always)]
    fn default() -> DsAddr {
        DsAddr(0)
    }
}
#[doc = "Force Event"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceEvent(pub u32);
impl ForceEvent {
    #[doc = "Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub const fn fevtac12ne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub fn set_fevtac12ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub const fn fevtac12toe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub fn set_fevtac12toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub const fn fevtac12ce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub fn set_fevtac12ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub const fn fevtac12ebe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub fn set_fevtac12ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub const fn fevtac12ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub fn set_fevtac12ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub const fn fevtcnibac12e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub fn set_fevtcnibac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Force Event Command Time Out Error"]
    #[inline(always)]
    pub const fn fevtctoe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Command Time Out Error"]
    #[inline(always)]
    pub fn set_fevtctoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force Event Command CRC Error"]
    #[inline(always)]
    pub const fn fevtcce(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Command CRC Error"]
    #[inline(always)]
    pub fn set_fevtcce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force Event Command End Bit Error"]
    #[inline(always)]
    pub const fn fevtcebe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Command End Bit Error"]
    #[inline(always)]
    pub fn set_fevtcebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force Event Command Index Error"]
    #[inline(always)]
    pub const fn fevtcie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Command Index Error"]
    #[inline(always)]
    pub fn set_fevtcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Force Event Data Time Out Error"]
    #[inline(always)]
    pub const fn fevtdtoe(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Data Time Out Error"]
    #[inline(always)]
    pub fn set_fevtdtoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Force Event Data CRC Error"]
    #[inline(always)]
    pub const fn fevtdce(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Data CRC Error"]
    #[inline(always)]
    pub fn set_fevtdce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Force Event Data End Bit Error"]
    #[inline(always)]
    pub const fn fevtdebe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Data End Bit Error"]
    #[inline(always)]
    pub fn set_fevtdebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub const fn fevtac12e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub fn set_fevtac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Force Tuning Error"]
    #[inline(always)]
    pub const fn fevttne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Force Tuning Error"]
    #[inline(always)]
    pub fn set_fevttne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Force Event DMA Error"]
    #[inline(always)]
    pub const fn fevtdmae(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event DMA Error"]
    #[inline(always)]
    pub fn set_fevtdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Force Event Card Interrupt"]
    #[inline(always)]
    pub const fn fevtcint(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force Event Card Interrupt"]
    #[inline(always)]
    pub fn set_fevtcint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ForceEvent {
    #[inline(always)]
    fn default() -> ForceEvent {
        ForceEvent(0)
    }
}
#[doc = "Host Controller Capabilities"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtrlCap(pub u32);
impl HostCtrlCap {
    #[doc = "SDR50 support"]
    #[inline(always)]
    pub const fn sdr50_support(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDR50 support"]
    #[inline(always)]
    pub fn set_sdr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDR104 support"]
    #[inline(always)]
    pub const fn sdr104_support(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDR104 support"]
    #[inline(always)]
    pub fn set_sdr104_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDR50 support"]
    #[inline(always)]
    pub const fn ddr50_support(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDR50 support"]
    #[inline(always)]
    pub fn set_ddr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Counter for Retuning"]
    #[inline(always)]
    pub const fn time_count_retuning(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Time Counter for Retuning"]
    #[inline(always)]
    pub fn set_time_count_retuning(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Use Tuning for SDR50"]
    #[inline(always)]
    pub const fn use_tuning_sdr50(&self) -> super::vals::UseTuningSdr50 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::UseTuningSdr50::from_bits(val as u8)
    }
    #[doc = "Use Tuning for SDR50"]
    #[inline(always)]
    pub fn set_use_tuning_sdr50(&mut self, val: super::vals::UseTuningSdr50) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Retuning Mode"]
    #[inline(always)]
    pub const fn retuning_mode(&self) -> super::vals::RetuningMode {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::RetuningMode::from_bits(val as u8)
    }
    #[doc = "Retuning Mode"]
    #[inline(always)]
    pub fn set_retuning_mode(&mut self, val: super::vals::RetuningMode) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Max Block Length"]
    #[inline(always)]
    pub const fn mbl(&self) -> super::vals::Mbl {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbl::from_bits(val as u8)
    }
    #[doc = "Max Block Length"]
    #[inline(always)]
    pub fn set_mbl(&mut self, val: super::vals::Mbl) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "ADMA Support"]
    #[inline(always)]
    pub const fn admas(&self) -> super::vals::Admas {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Admas::from_bits(val as u8)
    }
    #[doc = "ADMA Support"]
    #[inline(always)]
    pub fn set_admas(&mut self, val: super::vals::Admas) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Speed Support"]
    #[inline(always)]
    pub const fn hss(&self) -> super::vals::Hss {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Hss::from_bits(val as u8)
    }
    #[doc = "High Speed Support"]
    #[inline(always)]
    pub fn set_hss(&mut self, val: super::vals::Hss) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA Support"]
    #[inline(always)]
    pub const fn dmas(&self) -> super::vals::Dmas {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmas::from_bits(val as u8)
    }
    #[doc = "DMA Support"]
    #[inline(always)]
    pub fn set_dmas(&mut self, val: super::vals::Dmas) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Suspend / Resume Support"]
    #[inline(always)]
    pub const fn srs(&self) -> super::vals::Srs {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Srs::from_bits(val as u8)
    }
    #[doc = "Suspend / Resume Support"]
    #[inline(always)]
    pub fn set_srs(&mut self, val: super::vals::Srs) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Voltage Support 3.3V"]
    #[inline(always)]
    pub const fn vs33(&self) -> super::vals::Vs33 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Vs33::from_bits(val as u8)
    }
    #[doc = "Voltage Support 3.3V"]
    #[inline(always)]
    pub fn set_vs33(&mut self, val: super::vals::Vs33) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Voltage Support 3.0 V"]
    #[inline(always)]
    pub const fn vs30(&self) -> super::vals::Vs30 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Vs30::from_bits(val as u8)
    }
    #[doc = "Voltage Support 3.0 V"]
    #[inline(always)]
    pub fn set_vs30(&mut self, val: super::vals::Vs30) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Voltage Support 1.8 V"]
    #[inline(always)]
    pub const fn vs18(&self) -> super::vals::Vs18 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Vs18::from_bits(val as u8)
    }
    #[doc = "Voltage Support 1.8 V"]
    #[inline(always)]
    pub fn set_vs18(&mut self, val: super::vals::Vs18) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for HostCtrlCap {
    #[inline(always)]
    fn default() -> HostCtrlCap {
        HostCtrlCap(0)
    }
}
#[doc = "Interrupt Signal Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSignalEn(pub u32);
impl IntSignalEn {
    #[doc = "Command Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn ccien(&self) -> super::vals::Ccien {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ccien::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn set_ccien(&mut self, val: super::vals::Ccien) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn tcien(&self) -> super::vals::Tcien {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcien::from_bits(val as u8)
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn set_tcien(&mut self, val: super::vals::Tcien) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub const fn bgeien(&self) -> super::vals::Bgeien {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bgeien::from_bits(val as u8)
    }
    #[doc = "Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn set_bgeien(&mut self, val: super::vals::Bgeien) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt Enable"]
    #[inline(always)]
    pub const fn dintien(&self) -> super::vals::Dintien {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dintien::from_bits(val as u8)
    }
    #[doc = "DMA Interrupt Enable"]
    #[inline(always)]
    pub fn set_dintien(&mut self, val: super::vals::Dintien) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn bwrien(&self) -> super::vals::Bwrien {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwrien::from_bits(val as u8)
    }
    #[doc = "Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn set_bwrien(&mut self, val: super::vals::Bwrien) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn brrien(&self) -> super::vals::Brrien {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brrien::from_bits(val as u8)
    }
    #[doc = "Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn set_brrien(&mut self, val: super::vals::Brrien) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub const fn cinsien(&self) -> super::vals::Cinsien {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cinsien::from_bits(val as u8)
    }
    #[doc = "Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn set_cinsien(&mut self, val: super::vals::Cinsien) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal Interrupt Enable"]
    #[inline(always)]
    pub const fn crmien(&self) -> super::vals::Crmien {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crmien::from_bits(val as u8)
    }
    #[doc = "Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn set_crmien(&mut self, val: super::vals::Crmien) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt Interrupt Enable"]
    #[inline(always)]
    pub const fn cintien(&self) -> super::vals::Cintien {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cintien::from_bits(val as u8)
    }
    #[doc = "Card Interrupt Interrupt Enable"]
    #[inline(always)]
    pub fn set_cintien(&mut self, val: super::vals::Cintien) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tuning Event Interrupt Enable"]
    #[inline(always)]
    pub const fn rteien(&self) -> super::vals::Rteien {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rteien::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Event Interrupt Enable"]
    #[inline(always)]
    pub fn set_rteien(&mut self, val: super::vals::Rteien) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning Pass Interrupt Enable"]
    #[inline(always)]
    pub const fn tpien(&self) -> super::vals::Tpien {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tpien::from_bits(val as u8)
    }
    #[doc = "Tuning Pass Interrupt Enable"]
    #[inline(always)]
    pub fn set_tpien(&mut self, val: super::vals::Tpien) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub const fn ctoeien(&self) -> super::vals::Ctoeien {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoeien::from_bits(val as u8)
    }
    #[doc = "Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_ctoeien(&mut self, val: super::vals::Ctoeien) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub const fn cceien(&self) -> super::vals::Cceien {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cceien::from_bits(val as u8)
    }
    #[doc = "Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_cceien(&mut self, val: super::vals::Cceien) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn cebeien(&self) -> super::vals::Cebeien {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebeien::from_bits(val as u8)
    }
    #[doc = "Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_cebeien(&mut self, val: super::vals::Cebeien) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub const fn cieien(&self) -> super::vals::Cieien {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cieien::from_bits(val as u8)
    }
    #[doc = "Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_cieien(&mut self, val: super::vals::Cieien) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub const fn dtoeien(&self) -> super::vals::Dtoeien {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoeien::from_bits(val as u8)
    }
    #[doc = "Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_dtoeien(&mut self, val: super::vals::Dtoeien) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub const fn dceien(&self) -> super::vals::Dceien {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dceien::from_bits(val as u8)
    }
    #[doc = "Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_dceien(&mut self, val: super::vals::Dceien) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn debeien(&self) -> super::vals::Debeien {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debeien::from_bits(val as u8)
    }
    #[doc = "Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_debeien(&mut self, val: super::vals::Debeien) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub const fn ac12eien(&self) -> super::vals::Ac12eien {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12eien::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_ac12eien(&mut self, val: super::vals::Ac12eien) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning Error Interrupt Enable"]
    #[inline(always)]
    pub const fn tneien(&self) -> super::vals::Tneien {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Tneien::from_bits(val as u8)
    }
    #[doc = "Tuning Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_tneien(&mut self, val: super::vals::Tneien) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA Error Interrupt Enable"]
    #[inline(always)]
    pub const fn dmaeien(&self) -> super::vals::Dmaeien {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmaeien::from_bits(val as u8)
    }
    #[doc = "DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn set_dmaeien(&mut self, val: super::vals::Dmaeien) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntSignalEn {
    #[inline(always)]
    fn default() -> IntSignalEn {
        IntSignalEn(0)
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Command Complete"]
    #[inline(always)]
    pub const fn cc(&self) -> super::vals::Cc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cc::from_bits(val as u8)
    }
    #[doc = "Command Complete"]
    #[inline(always)]
    pub fn set_cc(&mut self, val: super::vals::Cc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete"]
    #[inline(always)]
    pub const fn tc(&self) -> super::vals::Tc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tc::from_bits(val as u8)
    }
    #[doc = "Transfer Complete"]
    #[inline(always)]
    pub fn set_tc(&mut self, val: super::vals::Tc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event"]
    #[inline(always)]
    pub const fn bge(&self) -> super::vals::Bge {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bge::from_bits(val as u8)
    }
    #[doc = "Block Gap Event"]
    #[inline(always)]
    pub fn set_bge(&mut self, val: super::vals::Bge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt"]
    #[inline(always)]
    pub const fn dint(&self) -> super::vals::Dint {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dint::from_bits(val as u8)
    }
    #[doc = "DMA Interrupt"]
    #[inline(always)]
    pub fn set_dint(&mut self, val: super::vals::Dint) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready"]
    #[inline(always)]
    pub const fn bwr(&self) -> super::vals::Bwr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwr::from_bits(val as u8)
    }
    #[doc = "Buffer Write Ready"]
    #[inline(always)]
    pub fn set_bwr(&mut self, val: super::vals::Bwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready"]
    #[inline(always)]
    pub const fn brr(&self) -> super::vals::Brr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brr::from_bits(val as u8)
    }
    #[doc = "Buffer Read Ready"]
    #[inline(always)]
    pub fn set_brr(&mut self, val: super::vals::Brr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion"]
    #[inline(always)]
    pub const fn cins(&self) -> super::vals::Cins {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cins::from_bits(val as u8)
    }
    #[doc = "Card Insertion"]
    #[inline(always)]
    pub fn set_cins(&mut self, val: super::vals::Cins) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal"]
    #[inline(always)]
    pub const fn crm(&self) -> super::vals::Crm {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crm::from_bits(val as u8)
    }
    #[doc = "Card Removal"]
    #[inline(always)]
    pub fn set_crm(&mut self, val: super::vals::Crm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt"]
    #[inline(always)]
    pub const fn cint(&self) -> super::vals::Cint {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cint::from_bits(val as u8)
    }
    #[doc = "Card Interrupt"]
    #[inline(always)]
    pub fn set_cint(&mut self, val: super::vals::Cint) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn rte(&self) -> super::vals::Rte {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rte::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_rte(&mut self, val: super::vals::Rte) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn tp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_tp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command Timeout Error"]
    #[inline(always)]
    pub const fn ctoe(&self) -> super::vals::Ctoe {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoe::from_bits(val as u8)
    }
    #[doc = "Command Timeout Error"]
    #[inline(always)]
    pub fn set_ctoe(&mut self, val: super::vals::Ctoe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC Error"]
    #[inline(always)]
    pub const fn cce(&self) -> super::vals::Cce {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cce::from_bits(val as u8)
    }
    #[doc = "Command CRC Error"]
    #[inline(always)]
    pub fn set_cce(&mut self, val: super::vals::Cce) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error"]
    #[inline(always)]
    pub const fn cebe(&self) -> super::vals::Cebe {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebe::from_bits(val as u8)
    }
    #[doc = "Command End Bit Error"]
    #[inline(always)]
    pub fn set_cebe(&mut self, val: super::vals::Cebe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error"]
    #[inline(always)]
    pub const fn cie(&self) -> super::vals::Cie {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cie::from_bits(val as u8)
    }
    #[doc = "Command Index Error"]
    #[inline(always)]
    pub fn set_cie(&mut self, val: super::vals::Cie) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Timeout Error"]
    #[inline(always)]
    pub const fn dtoe(&self) -> super::vals::Dtoe {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoe::from_bits(val as u8)
    }
    #[doc = "Data Timeout Error"]
    #[inline(always)]
    pub fn set_dtoe(&mut self, val: super::vals::Dtoe) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error"]
    #[inline(always)]
    pub const fn dce(&self) -> super::vals::Dce {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dce::from_bits(val as u8)
    }
    #[doc = "Data CRC Error"]
    #[inline(always)]
    pub fn set_dce(&mut self, val: super::vals::Dce) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error"]
    #[inline(always)]
    pub const fn debe(&self) -> super::vals::Debe {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debe::from_bits(val as u8)
    }
    #[doc = "Data End Bit Error"]
    #[inline(always)]
    pub fn set_debe(&mut self, val: super::vals::Debe) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 Error"]
    #[inline(always)]
    pub const fn ac12e(&self) -> super::vals::Ac12e {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12e::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 Error"]
    #[inline(always)]
    pub fn set_ac12e(&mut self, val: super::vals::Ac12e) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn tne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_tne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA Error"]
    #[inline(always)]
    pub const fn dmae(&self) -> super::vals::Dmae {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmae::from_bits(val as u8)
    }
    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn set_dmae(&mut self, val: super::vals::Dmae) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
#[doc = "Interrupt Status Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatusEn(pub u32);
impl IntStatusEn {
    #[doc = "Command Complete Status Enable"]
    #[inline(always)]
    pub const fn ccsen(&self) -> super::vals::Ccsen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ccsen::from_bits(val as u8)
    }
    #[doc = "Command Complete Status Enable"]
    #[inline(always)]
    pub fn set_ccsen(&mut self, val: super::vals::Ccsen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer Complete Status Enable"]
    #[inline(always)]
    pub const fn tcsen(&self) -> super::vals::Tcsen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcsen::from_bits(val as u8)
    }
    #[doc = "Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn set_tcsen(&mut self, val: super::vals::Tcsen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block Gap Event Status Enable"]
    #[inline(always)]
    pub const fn bgesen(&self) -> super::vals::Bgesen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bgesen::from_bits(val as u8)
    }
    #[doc = "Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn set_bgesen(&mut self, val: super::vals::Bgesen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Interrupt Status Enable"]
    #[inline(always)]
    pub const fn dintsen(&self) -> super::vals::Dintsen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dintsen::from_bits(val as u8)
    }
    #[doc = "DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn set_dintsen(&mut self, val: super::vals::Dintsen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub const fn bwrsen(&self) -> super::vals::Bwrsen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwrsen::from_bits(val as u8)
    }
    #[doc = "Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn set_bwrsen(&mut self, val: super::vals::Bwrsen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub const fn brrsen(&self) -> super::vals::Brrsen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brrsen::from_bits(val as u8)
    }
    #[doc = "Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn set_brrsen(&mut self, val: super::vals::Brrsen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card Insertion Status Enable"]
    #[inline(always)]
    pub const fn cinssen(&self) -> super::vals::Cinssen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cinssen::from_bits(val as u8)
    }
    #[doc = "Card Insertion Status Enable"]
    #[inline(always)]
    pub fn set_cinssen(&mut self, val: super::vals::Cinssen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Removal Status Enable"]
    #[inline(always)]
    pub const fn crmsen(&self) -> super::vals::Crmsen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crmsen::from_bits(val as u8)
    }
    #[doc = "Card Removal Status Enable"]
    #[inline(always)]
    pub fn set_crmsen(&mut self, val: super::vals::Crmsen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card Interrupt Status Enable"]
    #[inline(always)]
    pub const fn cintsen(&self) -> super::vals::Cintsen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cintsen::from_bits(val as u8)
    }
    #[doc = "Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn set_cintsen(&mut self, val: super::vals::Cintsen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub const fn rtesen(&self) -> super::vals::Rtesen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rtesen::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub fn set_rtesen(&mut self, val: super::vals::Rtesen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning Pass Status Enable"]
    #[inline(always)]
    pub const fn tpsen(&self) -> super::vals::Tpsen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tpsen::from_bits(val as u8)
    }
    #[doc = "Tuning Pass Status Enable"]
    #[inline(always)]
    pub fn set_tpsen(&mut self, val: super::vals::Tpsen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command Timeout Error Status Enable"]
    #[inline(always)]
    pub const fn ctoesen(&self) -> super::vals::Ctoesen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoesen::from_bits(val as u8)
    }
    #[doc = "Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn set_ctoesen(&mut self, val: super::vals::Ctoesen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC Error Status Enable"]
    #[inline(always)]
    pub const fn ccesen(&self) -> super::vals::Ccesen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ccesen::from_bits(val as u8)
    }
    #[doc = "Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn set_ccesen(&mut self, val: super::vals::Ccesen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command End Bit Error Status Enable"]
    #[inline(always)]
    pub const fn cebesen(&self) -> super::vals::Cebesen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebesen::from_bits(val as u8)
    }
    #[doc = "Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn set_cebesen(&mut self, val: super::vals::Cebesen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command Index Error Status Enable"]
    #[inline(always)]
    pub const fn ciesen(&self) -> super::vals::Ciesen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ciesen::from_bits(val as u8)
    }
    #[doc = "Command Index Error Status Enable"]
    #[inline(always)]
    pub fn set_ciesen(&mut self, val: super::vals::Ciesen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Timeout Error Status Enable"]
    #[inline(always)]
    pub const fn dtoesen(&self) -> super::vals::Dtoesen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoesen::from_bits(val as u8)
    }
    #[doc = "Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn set_dtoesen(&mut self, val: super::vals::Dtoesen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC Error Status Enable"]
    #[inline(always)]
    pub const fn dcesen(&self) -> super::vals::Dcesen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dcesen::from_bits(val as u8)
    }
    #[doc = "Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn set_dcesen(&mut self, val: super::vals::Dcesen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data End Bit Error Status Enable"]
    #[inline(always)]
    pub const fn debesen(&self) -> super::vals::Debesen {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debesen::from_bits(val as u8)
    }
    #[doc = "Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn set_debesen(&mut self, val: super::vals::Debesen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub const fn ac12esen(&self) -> super::vals::Ac12esen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12esen::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn set_ac12esen(&mut self, val: super::vals::Ac12esen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning Error Status Enable"]
    #[inline(always)]
    pub const fn tnesen(&self) -> super::vals::Tnesen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Tnesen::from_bits(val as u8)
    }
    #[doc = "Tuning Error Status Enable"]
    #[inline(always)]
    pub fn set_tnesen(&mut self, val: super::vals::Tnesen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA Error Status Enable"]
    #[inline(always)]
    pub const fn dmaesen(&self) -> super::vals::Dmaesen {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmaesen::from_bits(val as u8)
    }
    #[doc = "DMA Error Status Enable"]
    #[inline(always)]
    pub fn set_dmaesen(&mut self, val: super::vals::Dmaesen) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatusEn {
    #[inline(always)]
    fn default() -> IntStatusEn {
        IntStatusEn(0)
    }
}
#[doc = "Mixer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MixCtrl(pub u32);
impl MixCtrl {
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Block Count Enable"]
    #[inline(always)]
    pub const fn bcen(&self) -> super::vals::Bcen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bcen::from_bits(val as u8)
    }
    #[doc = "Block Count Enable"]
    #[inline(always)]
    pub fn set_bcen(&mut self, val: super::vals::Bcen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub const fn ac12en(&self) -> super::vals::Ac12en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ac12en::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn set_ac12en(&mut self, val: super::vals::Ac12en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Dual Data Rate mode selection"]
    #[inline(always)]
    pub const fn ddr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Dual Data Rate mode selection"]
    #[inline(always)]
    pub fn set_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Transfer Direction Select"]
    #[inline(always)]
    pub const fn dtdsel(&self) -> super::vals::Dtdsel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dtdsel::from_bits(val as u8)
    }
    #[doc = "Data Transfer Direction Select"]
    #[inline(always)]
    pub fn set_dtdsel(&mut self, val: super::vals::Dtdsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi / Single Block Select"]
    #[inline(always)]
    pub const fn msbsel(&self) -> super::vals::Msbsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Msbsel::from_bits(val as u8)
    }
    #[doc = "Multi / Single Block Select"]
    #[inline(always)]
    pub fn set_msbsel(&mut self, val: super::vals::Msbsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "NIBBLE_POS"]
    #[inline(always)]
    pub const fn nibble_pos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NIBBLE_POS"]
    #[inline(always)]
    pub fn set_nibble_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub const fn ac23en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn set_ac23en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn exe_tune(&self) -> super::vals::ExeTune {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ExeTune::from_bits(val as u8)
    }
    #[doc = "Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_exe_tune(&mut self, val: super::vals::ExeTune) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "SMP_CLK_SEL"]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> super::vals::MixCtrlSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::MixCtrlSmpClkSel::from_bits(val as u8)
    }
    #[doc = "SMP_CLK_SEL"]
    #[inline(always)]
    pub fn set_smp_clk_sel(&mut self, val: super::vals::MixCtrlSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn auto_tune_en(&self) -> super::vals::AutoTuneEn {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::AutoTuneEn::from_bits(val as u8)
    }
    #[doc = "Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_auto_tune_en(&mut self, val: super::vals::AutoTuneEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn fbclk_sel(&self) -> super::vals::FbclkSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::FbclkSel::from_bits(val as u8)
    }
    #[doc = "Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_fbclk_sel(&mut self, val: super::vals::FbclkSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable HS400 Mode"]
    #[inline(always)]
    pub const fn hs400_mode(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable HS400 Mode"]
    #[inline(always)]
    pub fn set_hs400_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for MixCtrl {
    #[inline(always)]
    fn default() -> MixCtrl {
        MixCtrl(0)
    }
}
#[doc = "MMC Boot Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmcBoot(pub u32);
impl MmcBoot {
    #[doc = "DTOCV_ACK"]
    #[inline(always)]
    pub const fn dtocv_ack(&self) -> super::vals::DtocvAck {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DtocvAck::from_bits(val as u8)
    }
    #[doc = "DTOCV_ACK"]
    #[inline(always)]
    pub fn set_dtocv_ack(&mut self, val: super::vals::DtocvAck) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "BOOT_ACK"]
    #[inline(always)]
    pub const fn boot_ack(&self) -> super::vals::BootAck {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BootAck::from_bits(val as u8)
    }
    #[doc = "BOOT_ACK"]
    #[inline(always)]
    pub fn set_boot_ack(&mut self, val: super::vals::BootAck) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "BOOT_MODE"]
    #[inline(always)]
    pub const fn boot_mode(&self) -> super::vals::BootMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::BootMode::from_bits(val as u8)
    }
    #[doc = "BOOT_MODE"]
    #[inline(always)]
    pub fn set_boot_mode(&mut self, val: super::vals::BootMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "BOOT_EN"]
    #[inline(always)]
    pub const fn boot_en(&self) -> super::vals::BootEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::BootEn::from_bits(val as u8)
    }
    #[doc = "BOOT_EN"]
    #[inline(always)]
    pub fn set_boot_en(&mut self, val: super::vals::BootEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AUTO_SABG_EN"]
    #[inline(always)]
    pub const fn auto_sabg_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AUTO_SABG_EN"]
    #[inline(always)]
    pub fn set_auto_sabg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Disable Time Out"]
    #[inline(always)]
    pub const fn disable_time_out(&self) -> super::vals::DisableTimeOut {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DisableTimeOut::from_bits(val as u8)
    }
    #[doc = "Disable Time Out"]
    #[inline(always)]
    pub fn set_disable_time_out(&mut self, val: super::vals::DisableTimeOut) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "BOOT_BLK_CNT"]
    #[inline(always)]
    pub const fn boot_blk_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "BOOT_BLK_CNT"]
    #[inline(always)]
    pub fn set_boot_blk_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MmcBoot {
    #[inline(always)]
    fn default() -> MmcBoot {
        MmcBoot(0)
    }
}
#[doc = "Present State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresState(pub u32);
impl PresState {
    #[doc = "Command Inhibit (CMD)"]
    #[inline(always)]
    pub const fn cihb(&self) -> super::vals::Cihb {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cihb::from_bits(val as u8)
    }
    #[doc = "Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn set_cihb(&mut self, val: super::vals::Cihb) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Inhibit (DATA)"]
    #[inline(always)]
    pub const fn cdihb(&self) -> super::vals::Cdihb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cdihb::from_bits(val as u8)
    }
    #[doc = "Command Inhibit (DATA)"]
    #[inline(always)]
    pub fn set_cdihb(&mut self, val: super::vals::Cdihb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Data Line Active"]
    #[inline(always)]
    pub const fn dla(&self) -> super::vals::Dla {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dla::from_bits(val as u8)
    }
    #[doc = "Data Line Active"]
    #[inline(always)]
    pub fn set_dla(&mut self, val: super::vals::Dla) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SD Clock Stable"]
    #[inline(always)]
    pub const fn sdstb(&self) -> super::vals::Sdstb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sdstb::from_bits(val as u8)
    }
    #[doc = "SD Clock Stable"]
    #[inline(always)]
    pub fn set_sdstb(&mut self, val: super::vals::Sdstb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IPG_CLK Gated Off Internally"]
    #[inline(always)]
    pub const fn ipgoff(&self) -> super::vals::Ipgoff {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ipgoff::from_bits(val as u8)
    }
    #[doc = "IPG_CLK Gated Off Internally"]
    #[inline(always)]
    pub fn set_ipgoff(&mut self, val: super::vals::Ipgoff) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HCLK Gated Off Internally"]
    #[inline(always)]
    pub const fn hckoff(&self) -> super::vals::Hckoff {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hckoff::from_bits(val as u8)
    }
    #[doc = "HCLK Gated Off Internally"]
    #[inline(always)]
    pub fn set_hckoff(&mut self, val: super::vals::Hckoff) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IPG_PERCLK Gated Off Internally"]
    #[inline(always)]
    pub const fn peroff(&self) -> super::vals::Peroff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Peroff::from_bits(val as u8)
    }
    #[doc = "IPG_PERCLK Gated Off Internally"]
    #[inline(always)]
    pub fn set_peroff(&mut self, val: super::vals::Peroff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SD Clock Gated Off Internally"]
    #[inline(always)]
    pub const fn sdoff(&self) -> super::vals::Sdoff {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sdoff::from_bits(val as u8)
    }
    #[doc = "SD Clock Gated Off Internally"]
    #[inline(always)]
    pub fn set_sdoff(&mut self, val: super::vals::Sdoff) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Write Transfer Active"]
    #[inline(always)]
    pub const fn wta(&self) -> super::vals::Wta {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wta::from_bits(val as u8)
    }
    #[doc = "Write Transfer Active"]
    #[inline(always)]
    pub fn set_wta(&mut self, val: super::vals::Wta) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Read Transfer Active"]
    #[inline(always)]
    pub const fn rta(&self) -> super::vals::Rta {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rta::from_bits(val as u8)
    }
    #[doc = "Read Transfer Active"]
    #[inline(always)]
    pub fn set_rta(&mut self, val: super::vals::Rta) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Buffer Write Enable"]
    #[inline(always)]
    pub const fn bwen(&self) -> super::vals::Bwen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bwen::from_bits(val as u8)
    }
    #[doc = "Buffer Write Enable"]
    #[inline(always)]
    pub fn set_bwen(&mut self, val: super::vals::Bwen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Buffer Read Enable"]
    #[inline(always)]
    pub const fn bren(&self) -> super::vals::Bren {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Bren::from_bits(val as u8)
    }
    #[doc = "Buffer Read Enable"]
    #[inline(always)]
    pub fn set_bren(&mut self, val: super::vals::Bren) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub const fn rtr(&self) -> super::vals::Rtr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rtr::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn set_rtr(&mut self, val: super::vals::Rtr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tape Select Change Done"]
    #[inline(always)]
    pub const fn tscd(&self) -> super::vals::Tscd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Tscd::from_bits(val as u8)
    }
    #[doc = "Tape Select Change Done"]
    #[inline(always)]
    pub fn set_tscd(&mut self, val: super::vals::Tscd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub const fn cinst(&self) -> super::vals::Cinst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cinst::from_bits(val as u8)
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn set_cinst(&mut self, val: super::vals::Cinst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Card Detect Pin Level"]
    #[inline(always)]
    pub const fn cdpl(&self) -> super::vals::Cdpl {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cdpl::from_bits(val as u8)
    }
    #[doc = "Card Detect Pin Level"]
    #[inline(always)]
    pub fn set_cdpl(&mut self, val: super::vals::Cdpl) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Write Protect Switch Pin Level"]
    #[inline(always)]
    pub const fn wpspl(&self) -> super::vals::Wpspl {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Wpspl::from_bits(val as u8)
    }
    #[doc = "Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn set_wpspl(&mut self, val: super::vals::Wpspl) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CMD Line Signal Level"]
    #[inline(always)]
    pub const fn clsl(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CMD Line Signal Level"]
    #[inline(always)]
    pub fn set_clsl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DATA\\[7:0\\] Line Signal Level"]
    #[inline(always)]
    pub const fn dlsl(&self) -> super::vals::Dlsl {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Dlsl::from_bits(val as u8)
    }
    #[doc = "DATA\\[7:0\\] Line Signal Level"]
    #[inline(always)]
    pub fn set_dlsl(&mut self, val: super::vals::Dlsl) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for PresState {
    #[inline(always)]
    fn default() -> PresState {
        PresState(0)
    }
}
#[doc = "Protocol Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProtCtrl(pub u32);
impl ProtCtrl {
    #[doc = "Data Transfer Width"]
    #[inline(always)]
    pub const fn dtw(&self) -> super::vals::Dtw {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Dtw::from_bits(val as u8)
    }
    #[doc = "Data Transfer Width"]
    #[inline(always)]
    pub fn set_dtw(&mut self, val: super::vals::Dtw) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub const fn d3cd(&self) -> super::vals::D3cd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::D3cd::from_bits(val as u8)
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn set_d3cd(&mut self, val: super::vals::D3cd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Endian Mode"]
    #[inline(always)]
    pub const fn emode(&self) -> super::vals::Emode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Emode::from_bits(val as u8)
    }
    #[doc = "Endian Mode"]
    #[inline(always)]
    pub fn set_emode(&mut self, val: super::vals::Emode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Card Detect Test Level"]
    #[inline(always)]
    pub const fn cdtl(&self) -> super::vals::Cdtl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cdtl::from_bits(val as u8)
    }
    #[doc = "Card Detect Test Level"]
    #[inline(always)]
    pub fn set_cdtl(&mut self, val: super::vals::Cdtl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card Detect Signal Selection"]
    #[inline(always)]
    pub const fn cdss(&self) -> super::vals::Cdss {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cdss::from_bits(val as u8)
    }
    #[doc = "Card Detect Signal Selection"]
    #[inline(always)]
    pub fn set_cdss(&mut self, val: super::vals::Cdss) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA Select"]
    #[inline(always)]
    pub const fn dmasel(&self) -> super::vals::Dmasel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dmasel::from_bits(val as u8)
    }
    #[doc = "DMA Select"]
    #[inline(always)]
    pub fn set_dmasel(&mut self, val: super::vals::Dmasel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Stop At Block Gap Request"]
    #[inline(always)]
    pub const fn sabgreq(&self) -> super::vals::Sabgreq {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Sabgreq::from_bits(val as u8)
    }
    #[doc = "Stop At Block Gap Request"]
    #[inline(always)]
    pub fn set_sabgreq(&mut self, val: super::vals::Sabgreq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Continue Request"]
    #[inline(always)]
    pub const fn creq(&self) -> super::vals::Creq {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Creq::from_bits(val as u8)
    }
    #[doc = "Continue Request"]
    #[inline(always)]
    pub fn set_creq(&mut self, val: super::vals::Creq) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Read Wait Control"]
    #[inline(always)]
    pub const fn rwctl(&self) -> super::vals::Rwctl {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Rwctl::from_bits(val as u8)
    }
    #[doc = "Read Wait Control"]
    #[inline(always)]
    pub fn set_rwctl(&mut self, val: super::vals::Rwctl) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt At Block Gap"]
    #[inline(always)]
    pub const fn iabg(&self) -> super::vals::Iabg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Iabg::from_bits(val as u8)
    }
    #[doc = "Interrupt At Block Gap"]
    #[inline(always)]
    pub fn set_iabg(&mut self, val: super::vals::Iabg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub const fn rd_done_no_8clk(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub fn set_rd_done_no_8clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Read wait point"]
    #[inline(always)]
    pub const fn rd_wait_point(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Read wait point"]
    #[inline(always)]
    pub fn set_rd_wait_point(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub const fn wecint(&self) -> super::vals::Wecint {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Wecint::from_bits(val as u8)
    }
    #[doc = "Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn set_wecint(&mut self, val: super::vals::Wecint) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub const fn wecins(&self) -> super::vals::Wecins {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Wecins::from_bits(val as u8)
    }
    #[doc = "Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn set_wecins(&mut self, val: super::vals::Wecins) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub const fn wecrm(&self) -> super::vals::Wecrm {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Wecrm::from_bits(val as u8)
    }
    #[doc = "Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn set_wecrm(&mut self, val: super::vals::Wecrm) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub const fn burst_len_en(&self) -> super::vals::BurstLenEn {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::BurstLenEn::from_bits(val as u8)
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn set_burst_len_en(&mut self, val: super::vals::BurstLenEn) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub const fn non_exact_blk_rd(&self) -> super::vals::NonExactBlkRd {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::NonExactBlkRd::from_bits(val as u8)
    }
    #[doc = "NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub fn set_non_exact_blk_rd(&mut self, val: super::vals::NonExactBlkRd) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "RD_NO8CLK_EN"]
    #[inline(always)]
    pub const fn rd_no8clk_en(&self) -> super::vals::RdNo8clkEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RdNo8clkEn::from_bits(val as u8)
    }
    #[doc = "RD_NO8CLK_EN"]
    #[inline(always)]
    pub fn set_rd_no8clk_en(&mut self, val: super::vals::RdNo8clkEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ProtCtrl {
    #[inline(always)]
    fn default() -> ProtCtrl {
        ProtCtrl(0)
    }
}
#[doc = "Strobe DLL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StrobeDllCtrl(pub u32);
impl StrobeDllCtrl {
    #[doc = "Strobe DLL Control Enable"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Enable"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Strobe DLL Control Reset"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Reset"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Strobe DLL Control Slave Force Updated"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_slv_force_upd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Slave Force Updated"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_slv_force_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_slv_dly_target(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_slv_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_gate_update_0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_gate_update_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_gate_update_1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_gate_update_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Strobe DLL Control Slave Override"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_slv_override(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Control Slave Override"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_slv_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Strobe DLL Control Slave Override Value"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_slv_override_val(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Strobe DLL Control Slave Override Value"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_slv_override_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Strobe DLL Control Slave Update Interval"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_slv_update_int(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Strobe DLL Control Slave Update Interval"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_slv_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Strobe DLL Control Reference Update Interval"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl_ref_update_int(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Strobe DLL Control Reference Update Interval"]
    #[inline(always)]
    pub fn set_strobe_dll_ctrl_ref_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for StrobeDllCtrl {
    #[inline(always)]
    fn default() -> StrobeDllCtrl {
        StrobeDllCtrl(0)
    }
}
#[doc = "Strobe DLL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StrobeDllStatus(pub u32);
impl StrobeDllStatus {
    #[doc = "Strobe DLL Status Slave Lock"]
    #[inline(always)]
    pub const fn strobe_dll_sts_slv_lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Status Slave Lock"]
    #[inline(always)]
    pub fn set_strobe_dll_sts_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Strobe DLL Status Reference Lock"]
    #[inline(always)]
    pub const fn strobe_dll_sts_ref_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Strobe DLL Status Reference Lock"]
    #[inline(always)]
    pub fn set_strobe_dll_sts_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Strobe DLL Status Slave Select"]
    #[inline(always)]
    pub const fn strobe_dll_sts_slv_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Strobe DLL Status Slave Select"]
    #[inline(always)]
    pub fn set_strobe_dll_sts_slv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
    #[doc = "Strobe DLL Status Reference Select"]
    #[inline(always)]
    pub const fn strobe_dll_sts_ref_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Strobe DLL Status Reference Select"]
    #[inline(always)]
    pub fn set_strobe_dll_sts_ref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
}
impl Default for StrobeDllStatus {
    #[inline(always)]
    fn default() -> StrobeDllStatus {
        StrobeDllStatus(0)
    }
}
#[doc = "System Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrl(pub u32);
impl SysCtrl {
    #[doc = "Divisor"]
    #[inline(always)]
    pub const fn dvs(&self) -> super::vals::Dvs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Dvs::from_bits(val as u8)
    }
    #[doc = "Divisor"]
    #[inline(always)]
    pub fn set_dvs(&mut self, val: super::vals::Dvs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "SDCLK Frequency Select"]
    #[inline(always)]
    pub const fn sdclkfs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SDCLK Frequency Select"]
    #[inline(always)]
    pub fn set_sdclkfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Timeout Counter Value"]
    #[inline(always)]
    pub const fn dtocv(&self) -> super::vals::Dtocv {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Dtocv::from_bits(val as u8)
    }
    #[doc = "Data Timeout Counter Value"]
    #[inline(always)]
    pub fn set_dtocv(&mut self, val: super::vals::Dtocv) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "IPP_RST_N"]
    #[inline(always)]
    pub const fn ipp_rst_n(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IPP_RST_N"]
    #[inline(always)]
    pub fn set_ipp_rst_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Software Reset For ALL"]
    #[inline(always)]
    pub const fn rsta(&self) -> super::vals::Rsta {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Rsta::from_bits(val as u8)
    }
    #[doc = "Software Reset For ALL"]
    #[inline(always)]
    pub fn set_rsta(&mut self, val: super::vals::Rsta) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Software Reset For CMD Line"]
    #[inline(always)]
    pub const fn rstc(&self) -> super::vals::Rstc {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Rstc::from_bits(val as u8)
    }
    #[doc = "Software Reset For CMD Line"]
    #[inline(always)]
    pub fn set_rstc(&mut self, val: super::vals::Rstc) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Software Reset For DATA Line"]
    #[inline(always)]
    pub const fn rstd(&self) -> super::vals::Rstd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Rstd::from_bits(val as u8)
    }
    #[doc = "Software Reset For DATA Line"]
    #[inline(always)]
    pub fn set_rstd(&mut self, val: super::vals::Rstd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Initialization Active"]
    #[inline(always)]
    pub const fn inita(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization Active"]
    #[inline(always)]
    pub fn set_inita(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Reset Tuning"]
    #[inline(always)]
    pub const fn rstt(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Tuning"]
    #[inline(always)]
    pub fn set_rstt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SysCtrl {
    #[inline(always)]
    fn default() -> SysCtrl {
        SysCtrl(0)
    }
}
#[doc = "Tuning Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuningCtrl(pub u32);
impl TuningCtrl {
    #[doc = "TUNING_START_TAP"]
    #[inline(always)]
    pub const fn tuning_start_tap(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TUNING_START_TAP"]
    #[inline(always)]
    pub fn set_tuning_start_tap(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TUNING_COUNTER"]
    #[inline(always)]
    pub const fn tuning_counter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "TUNING_COUNTER"]
    #[inline(always)]
    pub fn set_tuning_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "TUNING_STEP"]
    #[inline(always)]
    pub const fn tuning_step(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "TUNING_STEP"]
    #[inline(always)]
    pub fn set_tuning_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "TUNING_WINDOW"]
    #[inline(always)]
    pub const fn tuning_window(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "TUNING_WINDOW"]
    #[inline(always)]
    pub fn set_tuning_window(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "STD_TUNING_EN"]
    #[inline(always)]
    pub const fn std_tuning_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "STD_TUNING_EN"]
    #[inline(always)]
    pub fn set_std_tuning_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for TuningCtrl {
    #[inline(always)]
    fn default() -> TuningCtrl {
        TuningCtrl(0)
    }
}
#[doc = "Vendor Specific Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec(pub u32);
impl VendSpec {
    #[doc = "Voltage selection"]
    #[inline(always)]
    pub const fn vselect(&self) -> super::vals::Vselect {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Vselect::from_bits(val as u8)
    }
    #[doc = "Voltage selection"]
    #[inline(always)]
    pub fn set_vselect(&mut self, val: super::vals::Vselect) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Check busy enable"]
    #[inline(always)]
    pub const fn ac12_wr_chkbusy_en(&self) -> super::vals::Ac12WrChkbusyEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ac12WrChkbusyEn::from_bits(val as u8)
    }
    #[doc = "Check busy enable"]
    #[inline(always)]
    pub fn set_ac12_wr_chkbusy_en(&mut self, val: super::vals::Ac12WrChkbusyEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Force CLK"]
    #[inline(always)]
    pub const fn frc_sdclk_on(&self) -> super::vals::FrcSdclkOn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FrcSdclkOn::from_bits(val as u8)
    }
    #[doc = "Force CLK"]
    #[inline(always)]
    pub fn set_frc_sdclk_on(&mut self, val: super::vals::FrcSdclkOn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CRC Check Disable"]
    #[inline(always)]
    pub const fn crc_chk_dis(&self) -> super::vals::CrcChkDis {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CrcChkDis::from_bits(val as u8)
    }
    #[doc = "CRC Check Disable"]
    #[inline(always)]
    pub fn set_crc_chk_dis(&mut self, val: super::vals::CrcChkDis) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Byte access"]
    #[inline(always)]
    pub const fn cmd_byte_en(&self) -> super::vals::CmdByteEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CmdByteEn::from_bits(val as u8)
    }
    #[doc = "Byte access"]
    #[inline(always)]
    pub fn set_cmd_byte_en(&mut self, val: super::vals::CmdByteEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for VendSpec {
    #[inline(always)]
    fn default() -> VendSpec {
        VendSpec(0)
    }
}
#[doc = "Vendor Specific 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec2(pub u32);
impl VendSpec2 {
    #[doc = "Card Interrupt Detection Test"]
    #[inline(always)]
    pub const fn card_int_d3_test(&self) -> super::vals::CardIntD3Test {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CardIntD3Test::from_bits(val as u8)
    }
    #[doc = "Card Interrupt Detection Test"]
    #[inline(always)]
    pub fn set_card_int_d3_test(&mut self, val: super::vals::CardIntD3Test) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TUNING_8bit_EN"]
    #[inline(always)]
    pub const fn tuning_8bit_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TUNING_8bit_EN"]
    #[inline(always)]
    pub fn set_tuning_8bit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TUNING_1bit_EN"]
    #[inline(always)]
    pub const fn tuning_1bit_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TUNING_1bit_EN"]
    #[inline(always)]
    pub fn set_tuning_1bit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TUNING_CMD_EN"]
    #[inline(always)]
    pub const fn tuning_cmd_en(&self) -> super::vals::TuningCmdEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TuningCmdEn::from_bits(val as u8)
    }
    #[doc = "TUNING_CMD_EN"]
    #[inline(always)]
    pub fn set_tuning_cmd_en(&mut self, val: super::vals::TuningCmdEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HS400 Write Clock Stop Enable"]
    #[inline(always)]
    pub const fn hs400_wr_clk_stop_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HS400 Write Clock Stop Enable"]
    #[inline(always)]
    pub fn set_hs400_wr_clk_stop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HS400 Read Clock Stop Enable"]
    #[inline(always)]
    pub const fn hs400_rd_clk_stop_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HS400 Read Clock Stop Enable"]
    #[inline(always)]
    pub fn set_hs400_rd_clk_stop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub const fn acmd23_argu2_en(&self) -> super::vals::Acmd23Argu2En {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Acmd23Argu2En::from_bits(val as u8)
    }
    #[doc = "Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn set_acmd23_argu2_en(&mut self, val: super::vals::Acmd23Argu2En) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "AHB BUS reset"]
    #[inline(always)]
    pub const fn ahb_rst(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "AHB BUS reset"]
    #[inline(always)]
    pub fn set_ahb_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for VendSpec2 {
    #[inline(always)]
    fn default() -> VendSpec2 {
        VendSpec2(0)
    }
}
#[doc = "Watermark Level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WtmkLvl(pub u32);
impl WtmkLvl {
    #[doc = "Read Watermark Level"]
    #[inline(always)]
    pub const fn rd_wml(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Watermark Level"]
    #[inline(always)]
    pub fn set_rd_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub const fn rd_brst_len(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn set_rd_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Write Watermark Level"]
    #[inline(always)]
    pub const fn wr_wml(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write Watermark Level"]
    #[inline(always)]
    pub fn set_wr_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub const fn wr_brst_len(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn set_wr_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for WtmkLvl {
    #[inline(always)]
    fn default() -> WtmkLvl {
        WtmkLvl(0)
    }
}
