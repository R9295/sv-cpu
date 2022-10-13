mod svcpu;

fn main() {
    let cpu = svcpu::CPU {
        ..Default::default()
    };
    println!("Hello, world!");
}
