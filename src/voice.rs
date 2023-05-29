use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

static mut VOICING: bool = true;


pub fn listen_voice_stop(){

    unsafe { VOICING = false };
}
pub fn listen_voice(){

    // 获取系统默认麦克风音量 有声音变化的话 至少隔100毫秒输出一次 声音音量范围最好可以量化到 0-100
    // 目前看到用的比较多的是 cpal 库 
    let host = cpal::default_host();
    // let device = host.default_input_device().expect("Failed to get default input device");
    let input: Vec<_> = host.input_devices().unwrap().collect();
    let device = input.get(1).unwrap();
    
    let config = device.default_input_config().expect("Failed to get default input config");
    let stream = device.build_input_stream(&config.into(), move |data: &[f32], _| {
        // dbg!( max(data) * 100. );
        let volume = (max(data) * 100.).round();
        println!( "{}", volume);
    }, |err| {
        dbg!(err);
    }, None).unwrap();
    stream.play().unwrap();

    std::thread::spawn(||{
        std::thread::sleep(std::time::Duration::from_secs(10));
        listen_voice_stop();
    });

    while unsafe { VOICING } {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    drop(stream);
}

fn max(slice: &[f32]) -> f32 {
    let mut max = 0.0;
    for &i in slice {
        if i > max {
            max = i;
        }
    }
    max
}