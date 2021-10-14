use core::convert::TryInto;

use super::App as _;
use crate::{Card, Result};

pub struct App {
    pub card: Card,
}

impl super::App for App {
    const RID: &'static [u8] = super::SOLOKEYS_RID;
    const PIX: &'static [u8] = super::ADMIN_PIX;

    fn new(uuid: Option<[u8; 16]>, name: Option<&str>) -> Result<Self> {
        Ok(Self {
            card: Self::connect(uuid, name)?,
        })
    }

    fn card(&mut self) -> &mut Card {
        &mut self.card
    }
}

impl App {
    pub const BOOT_TO_BOOTROM_COMMAND: u8 = 0x51;
    pub const REBOOT_COMMAND: u8 = 0x53;
    pub const VERSION_COMMAND: u8 = 0x61;
    pub const UUID_COMMAND: u8 = 0x62;

    pub fn boot_to_bootrom(&mut self) -> Result<()> {
        println!("Tap button on key...");
        // Rebooting can cause the connection to return error, which is ok here.
        self.call(Self::BOOT_TO_BOOTROM_COMMAND).map(drop).ok();
        Ok(())
    }

    pub fn reboot(&mut self) -> Result<()> {
        // Rebooting can cause the connection to return error, which is ok here.
        self.call(Self::REBOOT_COMMAND).map(drop).ok();
        Ok(())
    }

    pub fn uuid(&mut self) -> Result<u128> {
        let version_bytes = self.call(Self::UUID_COMMAND)?;
        let bytes: &[u8] = &version_bytes;
        bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 16 byte UUID, got {}", &hex::encode(bytes)))
            .map(|bytes| u128::from_be_bytes(bytes))
    }

    pub fn version(&mut self) -> Result<[u8; 4]> {
        let version_bytes = self.call(Self::VERSION_COMMAND)?;
        let bytes: &[u8] = &version_bytes;
        bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 4 bytes version, got {}", &hex::encode(bytes)))
    }
}
