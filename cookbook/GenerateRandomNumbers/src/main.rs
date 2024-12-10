use rand;
use rand::distr::{DistString, Distribution, StandardUniform};
use rand::Rng;

#[derive(Debug)]
#[allow(dead_code)] // 编译器会警告x，y没有使用，但是实际是用到的语法分析有问题？允许一下dead_code免去警告
struct Point {
    x: i32,
    y: i32,
}

// 实现Point的随机生成
impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.random();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    // 基于 rand 0.9 以上版本
    // 下面的代码本质上都是通过 rand::rng() 来生成随机值
    // 只有有些方便的封装，不用写这么长的代码
    // rand::rng().random::<u8>();
    println!("random u8: {}", rand::random::<u8>());
    println!("random u16: {}", rand::random::<u16>());
    println!("random u32: {}", rand::random::<u32>());
    println!("random u64: {}", rand::random::<u64>());
    println!("random u128: {}", rand::random::<u128>());
    println!("random f32: {}", rand::random::<f32>());
    println!("random f64: {}", rand::random::<f64>());

    // 随机一个范围
    println!("random range [0, 10): {}", rand::random_range(0..10));
    println!("random range bool 50% is true: {}", rand::random_bool(1.0 / 2.0));

    // 如果要多次随机 可以使用 Uniform，性能会有一点优势，随机次数不多不需要使用
    {
        let mut rng = rand::rng();
        let die = rand::distr::Uniform::try_from(1..7).unwrap();
        loop {
            let throw = rng.sample(die);
            println!("throw: {}", throw);
            if throw == 6 {
                println!("lucky is 6! exit;");
                break;
            }
        }
    }

    // 支持生成类型的随机值
    {
        let mut rng = rand::rng();
        println!("rng.random::<(u8, u32, f64)>: {:?}", rng.random::<(u8, u32, f64)>());
        println!("rng.random::<Point>: {:?}", rng.random::<Point>());
    }

    // 随机字符串生成
    {
        let mut rng = rand::rng();
        println!("random string of 20 length: {}", rand::distr::Alphanumeric.sample_string(&mut rng, 20));
    }

    // 指定字符集，生成随机字符串
    {
        let mut rng = rand::rng();
        const CHARSET: &str  = "中文字符随机串范围";
        const STRING_LEN: usize = 10;

        let random_string: String = (0..STRING_LEN).map(|_|{
            let index = rng.random_range(0..CHARSET.chars().count());
            CHARSET.chars().nth(index).unwrap()
        }).collect();
        println!("random string: {}", random_string);
    }

}
