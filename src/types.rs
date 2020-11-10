//! Rust Types.

use std::io::prelude::*;
use std::convert::From;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::Pg;
use postgis::ewkb::Point;
use crate::sql_types::*;

/*
 
    _____                                  _           
   / ____|                                | |          
  | |  __  ___  ___   __ _ _ __ __ _ _ __ | |__  _   _ 
  | | |_ |/ _ \/ _ \ / _` | '__/ _` | '_ \| '_ \| | | |
  | |__| |  __/ (_) | (_| | | | (_| | |_) | | | | |_| |
   \_____|\___|\___/ \__, |_|  \__,_| .__/|_| |_|\__, |
                      __/ |         | |           __/ |
                     |___/          |_|          |___/ 
 
*/
#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geography"]
pub struct GeogPoint {
	pub x: f64, // lon
	pub y: f64, // lat
	pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
	fn from(p: Point) -> Self {
		let Point { x, y, srid } = p;
		Self { x, y, srid }
	}
}
impl From<GeogPoint> for Point {
	fn from(p: GeogPoint) -> Self {
		let GeogPoint { x, y, srid } = p;
		Self { x, y, srid }
	}
}

impl FromSql<Geography, Pg> for GeogPoint {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		use std::io::Cursor;
		use postgis::ewkb::EwkbRead;
		let bytes = not_none!(bytes);
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
	}
}

impl ToSql<Geography, Pg> for GeogPoint {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
	}
}


/*
 
    _____                           _              
   / ____|                         | |             
  | |  __  ___  ___  _ __ ___   ___| |_ _ __ _   _ 
  | | |_ |/ _ \/ _ \| '_ ` _ \ / _ \ __| '__| | | |
  | |__| |  __/ (_) | | | | | |  __/ |_| |  | |_| |
   \_____|\___|\___/|_| |_| |_|\___|\__|_|   \__, |
                                              __/ |
                                             |___/ 
 
*/

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Geometry"]
pub struct GeomPoint {
	pub x: f64, // lon
	pub y: f64, // lat
	pub srid: Option<i32>,
}

impl From<Point> for GeomPoint {
	fn from(p: Point) -> Self {
		let Point { x, y, srid } = p;
		Self { x, y, srid }
	}
}
impl From<GeomPoint> for Point {
	fn from(p: GeomPoint) -> Self {
		let GeomPoint { x, y, srid } = p;
		Self { x, y, srid }
	}
}

impl FromSql<Geometry, Pg> for GeomPoint {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		use std::io::Cursor;
		use postgis::ewkb::EwkbRead;
		let bytes = not_none!(bytes);
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
	}
}

impl ToSql<Geometry, Pg> for GeomPoint {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
	}
}
