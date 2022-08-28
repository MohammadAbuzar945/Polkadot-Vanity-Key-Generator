use base58::*;
use core::iter::{once, repeat};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Address {
    _prefix: u8,
    key: [u8; 32],
    sum: [u8; 2],
}

pub const MAX_VANITY: usize = 20;
const POLKADOT: char = '0';
const DEFAULT_FILL: char = '0';

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Address {
    /// The vanity function generates a Polkadot address that contains the provided text filled
    /// with a character of choice and the correct checksum to make it a valid address
    /// ```
    /// # use keyless_vanity::Address;
    /// let addr = Address::with_vanity("HelloWorld", '1').expect("valid address");
    /// let expected = "1HeLLoWorLd1111111111111111111111111111111112kn";
    /// assert_eq!(addr.to_string(), expected);
    /// ```
    #[cfg_attr(feature = "web", wasm_bindgen(js_name = withVanity))]
    pub fn with_vanity(input: &str, fill_c: char) -> Option<Address> {
        input.len().le(&MAX_VANITY).then(|| ())?;
        let mut input = input.chars().map(replace_invalid);
        let fill_c = replace_invalid(fill_c);

        let addr = repeat(fill_c).take(46).map(|c| input.next().unwrap_or(c));
        // TODO: how to support other prefixes?
        let addr: String = once(POLKADOT).chain(addr).collect();
        let mut addr = addr.parse::<Address>().ok()?;

        addr.sum = {
            let h = ss58hash(&addr.as_ref()[..addr.as_ref().len() - addr.sum.len()]);
            h.as_bytes()[..2].try_into().unwrap()
        };
        Some(addr)
    }

    pub fn key(&self) -> Vec<u8> {
        self.key.into()
    }

    pub fn encode(&self) -> String {
        self.to_string()
    }
}
///Implementation of the Polkadot address format
impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        // SAFETY: TODO: seems safe but could use a second opinion
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}
///Implementation of the Polkadot address format
impl core::str::FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = s.from_base58().map_err(|_| ())?;
        if decoded.len() != 35 {
            return Err(());
        }

        let len = decoded.len();
        Ok(Address {
            _prefix: decoded[0],
            key: decoded[1..len - 2].try_into().expect("key fits"),
            sum: decoded[len - 2..].try_into().expect("bigger than 2 bytes"),
        })
    }
}
///Implementation of the Polkadot address format
impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref().to_base58())
    }
}

const fn replace_invalid(c: char) -> char {
    match c {
        'O' => 'o',
        'I' => 'i',
        'l' => 'L',
        c if !c.is_ascii_alphanumeric() || c == '0' => DEFAULT_FILL,
        _ => c,
    }
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    const PREFIX: &[u8] = b"SS58PRE";
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}

#[cfg(test)]
mod tests {
    use core::iter::repeat_with;

    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use rand::thread_rng;
    use shannon_entropy::shannon_entropy;

    #[test]
    fn entropy() {
        let min_control_entropy = [
            // secure randomly generated addresses
            "1296vf5iNqAuyERoE9ZpfiuW6srDfpvmUXWKzJEH7HCzLjpD",
            "13GEkmXzzXECQfRsEduX2ixkYTgoJ1KjxqE3z9yn5p7FoD9E",
            "13s7c6BJncif3vWgvPkFqgsMhGFdX2aUzWciWUJpz1KVKqhn",
            "14Am3HqaUYr5BYMfUfu3D7NM2oYSESX3KAJhdyyC8VfiCXmt",
            "15a6TPj9C9diabhfnN2wFGXFLrmtcvhRxHCqt1ZKLtaye5YD",
            "13jHYuYNDUXSsTGEChHg7WLDD6Mo62ShqoeLr5eUrkxEaXzC",
            "12DxG84p1rY9Z8dY4hEcMnYvNxXN9vwGUEqzLnEhjk7GL5QT",
            "1pCjWQFoJugh2yPoSfmA4NdETiMttLLKHnquV4uo4rwZGUv",
            "13iXJyKcrt1D3VA8L8nfcMJnjhzZdF1umgsJ8gQ7snJpVbeK",
            "14tXtSE56EKT4fu4HjP5mgbA5ftE1MbYaZfJrYRNiNHiKhQn",
            "14GgVBnmW5mv4xARpXiw7sXF761RDAiTGdQakBBH8RMHMEfk",
            "15SuenH944asaJeEKneLsvVa4ZYnt83t65FmvLLzSbmm4oHg",
            "12dyerfvM7rp5KAuvCJTkaT18sdu9BPgPeVgr8sAogbjU85n",
            "133F472Mcjy8QhFJaXSKDZX9RZmjFCiE5ZcsR9YgvtmARXfB",
            "16hS2mJirKsjAHUys9Mc3vwWiPLxMPtC3dqcTenxU1mAAboV",
            "15hykeuEgVCsnSE7pkCshVNszktBoKYAQt1dLF5yim5qt51e",
            "14ms9pZfkrXnr8FwhigLJEKXemUof3pmTDhUaVfq7k3DYdG5",
            "12rsYaHJJVCgdRmAJpnWYyugWVKEfr3mkw7GgpmympQSRvSM",
            "14G9ARuSAxwciZaaGs1dxejM4Ghv5iFanqQUNbiMk4aL7RkF",
            "145f5uGtquvExrzqPTq7kKKasKYimBvH94UtVML8h7JZLsz8",
        ]
        .iter()
        .map(|a| shannon_entropy(a))
        .reduce(f32::min)
        .unwrap();

        let mut rng = thread_rng();
        let max_control_vanity = repeat_with(|| Alphanumeric.sample_string(&mut rng, MAX_VANITY))
            .take(20)
            .map(|a| Address::with_vanity(&a, 'x').unwrap())
            .map(|a| shannon_entropy(&a.to_string()))
            .reduce(f32::max)
            .unwrap();

        assert!((min_control_entropy - max_control_vanity) > 1.0);
    }
}
