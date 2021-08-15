#[macro_use]
extern crate tracing;

mod bundler;
pub use bundler::AssetBundler;

mod plugin;
pub use bevy_asset::AssetIo;
pub use plugin::{BundledAssetIo, BundledAssetIoOptions, BundledAssetIoPlugin};

#[cfg(feature = "encryption")]
use aes::Aes128;
#[cfg(feature = "encryption")]
use block_modes::block_padding::Pkcs7;
#[cfg(feature = "encryption")]
use block_modes::{BlockMode, Cbc};

#[cfg(feature = "encryption")]
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const DEFAULT_ASSET_FOLDER: &str = "assets";
const DEFAULT_ASSET_BUNDLE_NAME: &str = "assets.bin";