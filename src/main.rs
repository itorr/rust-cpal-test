use rodio::{Device, Sink};
use std::time::{Duration, Instant};

fn main() {
    // 获取系统默认麦克风音量 有声音变化的话 至少隔100毫秒输出一次 声音音量范围最好可以量化到 0-100
    // 目前看到用的比较多的是 cpal 库 
}