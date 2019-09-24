use redis::Commands;
use crate::short::Short;

pub fn add_short(s: Short, con: &mut redis::Connection) -> redis::RedisResult<()> {
    let _ : () = con.set(s.token, s.target)?;
    Ok(())
}

pub fn get_short(token: &str, con: &mut redis::Connection) -> redis::RedisResult<Short> {
    Ok(Short{
        token: token.to_string(),
        target: con.get(token)?
    })
}
