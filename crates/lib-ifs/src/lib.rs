use std::{
    error::Error, fmt::Display, marker::PhantomData, ptr::NonNull, slice::from_raw_parts,
    string::FromUtf8Error,
};

use libc;

#[derive(Debug)]
pub struct InterfacesError(String);

impl Error for InterfacesError {}

impl Display for InterfacesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Interface(libc::if_nameindex);

impl Interface {
    pub fn index(&self) -> u32 {
        self.0.if_index
    }

    pub fn name(&self) -> String {
        let slice = if_name_to_slice(self.0.if_name);
        unsafe { String::from_utf8_unchecked(slice.to_vec()) }
    }

    pub fn try_name(&self) -> Result<String, FromUtf8Error> {
        let slice = if_name_to_slice(self.0.if_name);
        String::from_utf8(slice.to_vec())
    }
}

#[derive(Debug)]
pub struct Interfaces {
    ptr: NonNull<libc::if_nameindex>,
}

impl<'a> Drop for Interfaces {
    fn drop(&mut self) {
        unsafe { libc::if_freenameindex(self.ptr.as_ptr()) };
    }
}

impl<'a> IntoIterator for &'a Interfaces {
    type Item = &'a Interface;
    type IntoIter = InterfacesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        InterfacesIter {
            ptr: self.ptr.as_ptr(),
            marker: PhantomData,
        }
    }
}

pub struct InterfacesIter<'a> {
    ptr: *const libc::if_nameindex,
    marker: PhantomData<&'a Interface>,
}

impl<'a> Iterator for InterfacesIter<'a> {
    type Item = &'a Interface;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if (*self.ptr).if_index == 0 {
                None
            } else {
                let ifa = &*(self.ptr as *const Interface);
                self.ptr = self.ptr.add(1);
                Some(ifa)
            }
        }
    }
}

pub fn if_nameindex() -> Result<Interfaces, InterfacesError> {
    unsafe {
        let ifs = libc::if_nameindex();
        let ptr = match NonNull::new(ifs) {
            Some(ptr) => ptr,
            None => {
                return Err(InterfacesError(
                    "failed to retrieve network interfaces".into(),
                ))
            }
        };
        Ok(Interfaces { ptr })
    }
}

fn if_name_to_slice<'a>(if_name: *mut i8) -> &'a [u8] {
    let data = if_name as *const libc::c_char;
    let len = unsafe { libc::strlen(data) };
    unsafe { from_raw_parts(data as *const u8, len) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_interfaces() {
        let ifas = match if_nameindex() {
            Ok(ifas) => ifas,
            Err(err) => panic!("{err}"),
        };

        for ifa in ifas.into_iter() {
            println!("{}", ifa.name())
        }
    }
}
