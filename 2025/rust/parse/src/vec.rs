use glam::{I64Vec2, IVec2, U64Vec2, UVec2};
use nom::{bytes::complete::tag, character::complete, sequence::separated_pair, IResult};
use paste::paste;

macro_rules! impl_parse_vec2 {
    ($name: ident, $vec: ty, $int: ident) => {
        paste! {
            #[allow(clippy::missing_errors_doc)]
            pub fn [<parse_ $name>](input: &str) -> IResult<&str, $vec> {
                let (input, (x, y)) = separated_pair(complete::$int, tag(","), complete::$int)(input)?;
                Ok((input, <$vec>::new(x, y)))
            }

            #[allow(clippy::missing_errors_doc)]
            pub fn [<parse_ $name _res>](input: &str) -> anyhow::Result<$vec> {
                let (_, vec) = [<parse_ $name>](input).map_err(|e| e.to_owned())?;
                Ok(vec)
            }
        }
    };
}

impl_parse_vec2!(ivec2, IVec2, i32);
impl_parse_vec2!(uvec2, UVec2, u32);
impl_parse_vec2!(i64vec2, I64Vec2, i64);
impl_parse_vec2!(u64vec2, U64Vec2, u64);
