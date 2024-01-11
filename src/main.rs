use anyhow::Result;
use usbd_hid::descriptor::{KeyboardUsage, MediaKey};

const DEFAULT: [u32; 4] = [
  KeyboardUsage::KeyboardF13 as u32,
  KeyboardUsage::KeyboardF14 as u32,
  KeyboardUsage::KeyboardF15 as u32,
  KeyboardUsage::KeyboardF16 as u32,
];

fn main() -> Result<()> {
  let api = hidapi::HidApi::new()?;
  let device = api.open(0x7668, 0x0001)?;

  let (types, (modifiers, keys)): (Vec<u8>, (Vec<u8>, Vec<u8>)) = std::env::args()
    .skip(1)
    .take(4)
    .zip(DEFAULT)
    .map(|(arg, default)| {
      let [_, r#type, mods, key] = u32::from_str_radix(&arg, 16).unwrap_or(default).to_be_bytes();
      (r#type, (mods, key))
    })
    .unzip();
  assert_eq!(modifiers.len(), 4, "invalid argument count");

  device.write(&[types.clone(), keys.clone(), modifiers.clone()].concat())?;

  println!("flashed");
  types
    .iter()
    .zip(modifiers)
    .zip(keys)
    .for_each(|((r#type, modifier), key)| match r#type {
      0 => println!(" {modifier:08b} {key:02x} {:?}", KeyboardUsage::from(key)),
      1 => println!("    media {key:02x} {:?}", MediaKey::from(key)),
      _ => unreachable!(),
    });

  Ok(())
}
