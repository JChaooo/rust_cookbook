use rand::{self, distributions::Standard, prelude::Distribution, Rng, thread_rng};
use rand::distributions::{Alphanumeric, Uniform};
use rand_distr::{Normal, NormalError};

/**
 * @brief 测试 获取指定类型的随机数函数。
 * @param
 * @return 一个指定类型的随机数
 */
pub fn get_random_num<T>() -> T
    where
        Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    let num: T = rng.gen();
    num
}

/**
 * @brief 生成范围内随机数
 */
pub fn get_rang() {
    let mut rng = thread_rng();
    println!("Integer:{}", rng.gen_range(0..10));
    println!("Float:{}", rng.gen_range(0.0..10.0));

    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die:{}", throw);
        if throw == 6 {
            break;
        }
    }
}

/**
 * @brief 生成给定分布随机数
 */
pub fn get_sample_rang() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

/**
 * @brief 生成自定义随机值
 */
pub fn get_diy_rang() {
    let mut rng = thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

/**
 *@brief 从一组字母数字字符创建随机密码
 */
pub fn get_ascll_str() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
}

/**
 *@brief 从一组用户自定义字符创建随机密码
 */
pub fn get_pwd_from_div() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}", password);
}