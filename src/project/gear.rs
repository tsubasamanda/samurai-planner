#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GearStats {
    pub(crate) wd: u16,
    pub(crate) hp: u32,
    pub(crate) str: u16,
    pub(crate) crt: u16,
    pub(crate) dh: u16,
    pub(crate) det: u16,
    pub(crate) sks: u16
}