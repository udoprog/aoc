use lib::prelude::*;

#[entry(input = "d01.txt", expect = (999, 6099))]
fn main(mut input: IStr) -> Result<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut dial = 50;

    while let Some((d, l)) = input.line::<Option<(Dir, i32)>>()? {
        let m = l * d as i32;
        p2 += (dial + m + 99 * -i32::from(m < 0)).unsigned_abs() / 100;
        dial = (dial + m).rem_euclid(100);
        p1 += u32::from(dial == 0);
    }

    Ok((p1, p2))
}

#[derive(Debug)]
#[repr(i32)]
enum Dir {
    Left = -1,
    Right = 1,
}

lib::from_input! {
    |c: char| -> Dir {
        match c {
            'L' => Ok(Dir::Left),
            'R' => Ok(Dir::Right),
            c => bail!(c),
        }
    }
}
