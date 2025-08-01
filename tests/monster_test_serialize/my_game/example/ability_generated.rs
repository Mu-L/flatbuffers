// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
extern crate serde;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
// struct Ability, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Ability(pub [u8; 8]);
impl Default for Ability { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for Ability {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Ability")
      .field("id", &self.id())
      .field("distance", &self.distance())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Ability {}
impl<'a> flatbuffers::Follow<'a> for Ability {
  type Inner = &'a Ability;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    unsafe { <&'a Ability>::follow(buf, loc) }
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Ability {
  type Inner = &'a Ability;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    unsafe { flatbuffers::follow_cast_ref::<Ability>(buf, loc) }
  }
}
impl<'b> flatbuffers::Push for Ability {
    type Output = Ability;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = unsafe { ::core::slice::from_raw_parts(self as *const Ability as *const u8, <Self as flatbuffers::Push>::size()) };
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for Ability {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl Serialize for Ability {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("Ability", 2)?;
      s.serialize_field("id", &self.id())?;
      s.serialize_field("distance", &self.distance())?;
    s.end()
  }
}

impl<'a> Ability {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    id: u32,
    distance: u32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_id(id);
    s.set_distance(distance);
    s
  }

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Example.Ability"
  }

  pub fn id(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_id(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  #[inline]
  pub fn key_compare_less_than(&self, o: &Ability) -> bool {
    self.id() < o.id()
  }

  #[inline]
  pub fn key_compare_with_value(&self, val: u32) -> ::core::cmp::Ordering {
    let key = self.id();
    key.cmp(&val)
  }
  pub fn distance(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_distance(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn unpack(&self) -> AbilityT {
    AbilityT {
      id: self.id(),
      distance: self.distance(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbilityT {
  pub id: u32,
  pub distance: u32,
}
impl AbilityT {
  pub fn pack(&self) -> Ability {
    Ability::new(
      self.id,
      self.distance,
    )
  }
}

