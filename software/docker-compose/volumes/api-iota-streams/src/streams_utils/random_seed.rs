use rand::Rng;
pub fn randomSeed(size :usize) -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
  let SEED_LEN: usize = size;
  let mut rng = rand::thread_rng();
  let seed: String = (0..SEED_LEN)
    .map(|_| {
      let idx = rng.gen_range(0, CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();
  seed
}