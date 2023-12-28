use anyhow::Result;
use usbd_hid::descriptor::KeyboardUsage;

fn main() -> Result<()> {
  let api = hidapi::HidApi::new()?;
  let device = api.open(0x7668, 0x0001)?;

  let args: Vec<u8> = std::env::args()
    .skip(1)
    .map(|arg| u8::from_str_radix(&arg, 16).unwrap())
    .collect();
  let buf: Vec<u8> = [
    KeyboardUsage::KeyboardF13,
    KeyboardUsage::KeyboardF14,
    KeyboardUsage::KeyboardF15,
    KeyboardUsage::KeyboardF16,
  ]
  .iter()
  .enumerate()
  .map(|(i, key)| if i < args.len() { args[i] } else { *key as u8 })
  .collect();
  device.write(&buf)?;
  println!("flashed");
  buf
    .iter()
    .for_each(|keycode| println!(" {:?}", KeyboardUsage::from(*keycode)));

  Ok(())
}
