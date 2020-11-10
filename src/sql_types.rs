//! SQL Types.

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geography")]
pub struct Geography;

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geometry")]
pub struct Geometry;
