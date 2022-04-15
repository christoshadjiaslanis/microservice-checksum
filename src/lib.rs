

#[macro_use]
extern crate rocket;

use crc::{crc16, Hasher16, crc32, Hasher32, crc64, Hasher64};
use rocket::http::{RawStr};
use rocket::{Build, form, Rocket};
use rocket::form::{DataField, Form, FromFormField, ValueField};
use rocket::request::FromRequest;

#[derive(Debug)]
enum Crc16Polynomial {
    X25,
    Usb,
    Custom(u16),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Crc16Polynomial {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value {
            "usb" => Ok(Crc16Polynomial::Usb),
            "x25" => Ok(Crc16Polynomial::X25),
            custom => {
                match str::parse::<u16>(custom) {
                    Ok(n) => Ok(Crc16Polynomial::Custom(n)),
                    Err(_) => Err(form::Error::validation("unknown polynomial"))?,
                }
            },
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("parse from a value or use default impl")
    }
}

#[derive(Debug, FromForm)]
struct Crc16Options {
    polynomial: Crc16Polynomial,
}

#[post("/crc16?<options..>", data = "<payload>")]
fn crc16_endpoint(payload: Vec<u8>, options: Option<Crc16Options>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc16Polynomial::X25 => crc::crc16::X25,
            Crc16Polynomial::Usb => crc::crc16::USB,
            Crc16Polynomial::Custom(n) => n,
        };

        let mut digest = crc16::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum16()
    } else {
        crc16::checksum_usb(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}


#[derive(Debug)]
enum Crc32Polynomial {
    Ieee,
    Castagnoli,
    Koopman,
    Custom(u32),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Crc32Polynomial {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value {
            "ieee" => Ok(Crc32Polynomial::Ieee),
            "castagnoli" => Ok(Crc32Polynomial::Castagnoli),
            "koopman" => Ok(Crc32Polynomial::Koopman),
            custom => {
                match str::parse::<u32>(custom) {
                    Ok(n) => Ok(Crc32Polynomial::Custom(n)),
                    Err(_) => Err(form::Error::validation("unknown polynomial"))?,
                }
            },
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("parse from a value or use default impl")
    }
}

#[derive(Debug, FromForm)]
struct Crc32Options {
    polynomial: Crc32Polynomial,
}

#[post("/crc32?<options..>", data = "<payload>")]
fn crc32_endpoint(payload: Vec<u8>, options: Option<Crc32Options>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc32Polynomial::Ieee => crc::crc32::IEEE,
            Crc32Polynomial::Castagnoli => crc::crc32::CASTAGNOLI,
            Crc32Polynomial::Koopman => crc::crc32::KOOPMAN,
            Crc32Polynomial::Custom(n) => n,
        };

        let mut digest = crc32::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum32()

    } else {
        crc32::checksum_ieee(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}

#[derive(Debug)]
enum Crc64Polynomial {
    Ecma,
    Iso,
    Custom(u64),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Crc64Polynomial {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value {
            "ecma" => Ok(Crc64Polynomial::Ecma),
            "iso" => Ok(Crc64Polynomial::Iso),
            custom => {
                match str::parse::<u64>(custom) {
                    Ok(n) => Ok(Crc64Polynomial::Custom(n)),
                    Err(_) => Err(form::Error::validation("unknown polynomial"))?,
                }
            },
        }    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("parse from a value or use default impl")
    }
}

#[derive(Debug, FromForm)]
struct Crc64Options {
    polynomial: Crc64Polynomial,
}

#[post("/crc64?<options..>", data = "<payload>")]
fn crc64_endpoint(payload: Vec<u8>, options: Option<Crc64Options>) -> String {
    let checksum = if let Some(choice) = options {
        let polynomial = match choice.polynomial {
            Crc64Polynomial::Ecma => crc::crc64::ECMA,
            Crc64Polynomial::Iso => crc::crc64::ISO,
            Crc64Polynomial::Custom(n) => n,
        };

        let mut digest = crc64::Digest::new(polynomial);
        digest.write(payload.as_slice());
        digest.sum64()
    } else {
        crc64::checksum_iso(payload.as_slice())
    };
    format!("{}\r\n", checksum)
}


#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    let rocket = rocket::build().mount("/", routes![crc16_endpoint, crc32_endpoint, crc64_endpoint]);

    Ok(rocket)
}