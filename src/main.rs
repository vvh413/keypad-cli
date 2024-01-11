use anyhow::Result;
use usbd_hid::descriptor::KeyboardUsage;

const DEFAULT: [u16; 4] = [
  KeyboardUsage::KeyboardF13 as u16,
  KeyboardUsage::KeyboardF14 as u16,
  KeyboardUsage::KeyboardF15 as u16,
  KeyboardUsage::KeyboardF16 as u16,
];

fn main() -> Result<()> {
  let api = hidapi::HidApi::new()?;
  let device = api.open(0x7668, 0x0001)?;

  let (modifiers, keys): (Vec<u8>, Vec<u8>) = std::env::args()
    .skip(1)
    .take(4)
    .zip(DEFAULT)
    .map(|(arg, default)| {
      let key = u16::from_str_radix(&arg, 16).unwrap_or(default);
      ((key >> 8) as u8, (key & 0xff) as u8)
    })
    .unzip();
  assert_eq!(modifiers.len(), 4, "invalid argument count");

  device.write(&[keys.clone(), modifiers.clone()].concat())?;

  println!("flashed");
  modifiers
    .iter()
    .zip(keys)
    .for_each(|(modifier, key)| println!(" {modifier:08b} {key:02x} {:?}", KeyboardUsage::from(key)));

  Ok(())
}
